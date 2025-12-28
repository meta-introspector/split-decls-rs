macro_rules! deps {
    () => {
        DefDatabase!();
    };
}

macro_rules! include_macro_invoc {
    () => {
        deps!();
        fn include_macro_invoc (db : & dyn DefDatabase , krate : Crate ,) -> Arc < [(MacroCallId , EditionedFileId)] > { crate_def_map (db , krate) . modules . values () . flat_map (| m | m . scope . iter_macro_invoc ()) . filter_map (| invoc | { db . lookup_intern_macro_call (* invoc . 1) . include_file_id (db , * invoc . 1) . map (| x | (* invoc . 1 , x)) }) . collect () }
    };
}

include_macro_invoc!()