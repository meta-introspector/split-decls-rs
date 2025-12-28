macro_rules! deps {
    () => {
        CallHierarchyConfig!();
        CallLocations!();
        CallItem!();
    };
}

macro_rules! incoming_calls {
    () => {
        deps!();
        pub (crate) fn incoming_calls (db : & RootDatabase , config : & CallHierarchyConfig < '_ > , FilePosition { file_id , offset } : FilePosition ,) -> Option < Vec < CallItem > > { let sema = & Semantics :: new (db) ; let file = sema . parse_guess_edition (file_id) ; let file = file . syntax () ; let mut calls = CallLocations :: default () ; let references = sema . find_nodes_at_offset_with_descend (file , offset) . filter_map (move | node | match node { ast :: NameLike :: NameRef (name_ref) => match NameRefClass :: classify (sema , & name_ref) ? { NameRefClass :: Definition (def @ Definition :: Function (_) , _) => Some (def) , _ => None , } , ast :: NameLike :: Name (name) => match NameClass :: classify (sema , & name) ? { NameClass :: Definition (def @ Definition :: Function (_)) => Some (def) , _ => None , } , ast :: NameLike :: Lifetime (_) => None , }) . flat_map (| func | func . usages (sema) . all ()) ; for (_ , references) in references { let references = references . iter () . filter_map (| FileReference { name , .. } | name . as_name_ref ()) ; for name in references { let def_nav = sema . ancestors_with_macros (name . syntax () . clone ()) . find_map (| node | { let def = ast :: Fn :: cast (node) . and_then (| fn_ | sema . to_def (& fn_)) ? ; def . try_to_nav (sema) . map (| nav | (def , nav)) }) ; if let Some ((def , nav)) = def_nav { if config . exclude_tests && def . is_test (db) { continue ; } let range = sema . original_range (name . syntax ()) ; calls . add (nav . call_site , range . into_file_id (db)) ; if let Some (other) = nav . def_site { calls . add (other , range . into_file_id (db)) ; } } } } Some (calls . into_items ()) }
    };
}

incoming_calls!()