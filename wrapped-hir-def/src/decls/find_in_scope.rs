macro_rules! deps {
    () => {
        DefMap!();
        DefDatabase!();
        ModuleId!();
        ItemInNs!();
    };
}

macro_rules! find_in_scope {
    () => {
        deps!();
        fn find_in_scope (db : & dyn DefDatabase , def_map : & DefMap , from : ModuleId , item : ItemInNs , ignore_local_imports : bool ,) -> Option < Name > { def_map . with_ancestor_maps (db , from . local_id , & mut | def_map , local_id | { def_map [local_id] . scope . names_of (item , | name , _ , declared | { (declared || ! ignore_local_imports) . then (| | name . clone ()) }) }) }
    };
}

find_in_scope!()