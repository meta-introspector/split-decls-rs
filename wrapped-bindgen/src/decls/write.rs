macro_rules! deps {
    () => {
        Interface!();
        TypeMap!();
        Index!();
        CppInterface!();
        Type!();
    };
}

macro_rules! write {
    () => {
        deps!();
        # [doc (hidden)] pub fn write (types : & TypeMap , output : & str) { let mut feature_index = Index :: new () ; let mut all_types : Vec < _ > = types . values () . flatten () . collect () ; all_types . sort_by (| a , b | { let a_name = a . type_name () ; let b_name = b . type_name () ; (a_name . namespace () , a_name . name ()) . cmp (& (b_name . namespace () , b_name . name ())) }) ; for ty in all_types { let type_deps = ty . dependencies () ; let type_name = ty . type_name () ; let namespace = type_name . namespace () ; if namespace . is_empty () { continue ; } let features : BTreeSet < String > = type_deps . keys () . filter (| tn | ! EXCLUDED_NAMESPACES . contains (& tn . namespace ())) . map (| tn | tn . namespace () . to_string ()) . chain (std :: iter :: once (namespace . to_string ())) . collect () ; feature_index . add_item (namespace , type_name . name () , features) ; let methods = match ty { Type :: CppInterface (ty) => { let interface_name = ty . def . name () ; Some ((ty . def . methods () , interface_name)) } Type :: Interface (ty) => { let interface_name = ty . def . name () ; Some ((ty . def . methods () , interface_name)) } _ => None , } ; if let Some ((methods , interface_name)) = methods { for method in methods { let method_deps = method . signature (namespace , & []) . dependencies () ; let method_features : BTreeSet < String > = method_deps . keys () . filter (| tn | ! EXCLUDED_NAMESPACES . contains (& tn . namespace ())) . map (| tn | tn . namespace () . to_string ()) . chain (std :: iter :: once (namespace . to_string ())) . collect () ; let scoped_name = format ! ("{}.{}" , interface_name , method . name ()) ; feature_index . add_item (namespace , & scoped_name , method_features) ; } } } write_to_file (output , serde_json :: to_string (& feature_index) . unwrap ()) ; }
    };
}

write!();