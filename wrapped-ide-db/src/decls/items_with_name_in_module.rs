macro_rules! deps {
    () => {
        Query!();
        RootDatabase!();
        SymbolIndex!();
    };
}

macro_rules! items_with_name_in_module {
    () => {
        deps!();
        # [doc = " Searches for importable items with the given name in the crate and its dependencies."] pub fn items_with_name_in_module < T > (db : & RootDatabase , module : Module , name : NameToImport , assoc_item_search : AssocSearchMode , mut cb : impl FnMut (ItemInNs) -> ControlFlow < T > ,) -> Option < T > { let _p = tracing :: info_span ! ("items_with_name_in" , name = name . text () , assoc_item_search = ? assoc_item_search , ? module) . entered () ; let prefix = matches ! (name , NameToImport :: Prefix (..)) ; let local_query = match name { NameToImport :: Prefix (exact_name , case_sensitive) | NameToImport :: Exact (exact_name , case_sensitive) => { let mut local_query = symbol_index :: Query :: new (exact_name) ; local_query . assoc_search_mode (assoc_item_search) ; if prefix { local_query . prefix () ; } else { local_query . exact () ; } if case_sensitive { local_query . case_sensitive () ; } local_query } NameToImport :: Fuzzy (fuzzy_search_string , case_sensitive) => { let mut local_query = symbol_index :: Query :: new (fuzzy_search_string) ; local_query . fuzzy () ; local_query . assoc_search_mode (assoc_item_search) ; if case_sensitive { local_query . case_sensitive () ; } local_query } } ; local_query . search (& [SymbolIndex :: module_symbols (db , module)] , | local_candidate | { cb (match local_candidate . def { hir :: ModuleDef :: Macro (macro_def) => ItemInNs :: Macros (macro_def) , def => ItemInNs :: from (def) , }) }) }
    };
}

items_with_name_in_module!();