macro_rules! deps {
    () => {
        Query!();
        RootDatabase!();
    };
}

macro_rules! items_with_name {
    () => {
        deps!();
        # [doc = " Searches for importable items with the given name in the crate and its dependencies."] pub fn items_with_name (db : & RootDatabase , krate : Crate , name : NameToImport , assoc_item_search : AssocSearchMode ,) -> impl Iterator < Item = (ItemInNs , Complete) > { let _p = tracing :: info_span ! ("items_with_name" , name = name . text () , assoc_item_search = ? assoc_item_search , crate = ? krate . display_name (db) . map (| name | name . to_string ())) . entered () ; let prefix = matches ! (name , NameToImport :: Prefix (..)) ; let (local_query , external_query) = match name { NameToImport :: Prefix (exact_name , case_sensitive) | NameToImport :: Exact (exact_name , case_sensitive) => { let mut local_query = symbol_index :: Query :: new (exact_name . clone ()) ; local_query . assoc_search_mode (assoc_item_search) ; let mut external_query = import_map :: Query :: new (exact_name) . assoc_search_mode (assoc_item_search) ; if prefix { local_query . prefix () ; external_query = external_query . prefix () ; } else { local_query . exact () ; external_query = external_query . exact () ; } if case_sensitive { local_query . case_sensitive () ; external_query = external_query . case_sensitive () ; } (local_query , external_query) } NameToImport :: Fuzzy (fuzzy_search_string , case_sensitive) => { let mut local_query = symbol_index :: Query :: new (fuzzy_search_string . clone ()) ; local_query . fuzzy () ; local_query . assoc_search_mode (assoc_item_search) ; let mut external_query = import_map :: Query :: new (fuzzy_search_string) . fuzzy () . assoc_search_mode (assoc_item_search) ; if case_sensitive { local_query . case_sensitive () ; external_query = external_query . case_sensitive () ; } (local_query , external_query) } } ; find_items (db , krate , local_query , external_query) }
    };
}

items_with_name!()