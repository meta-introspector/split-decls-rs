macro_rules! get_object_file_path {
    () => {
        fn get_object_file_path (sess : & Session , name : & str , self_contained : bool) -> PathBuf { let file_path = sess . target_tlib_path . dir . join (name) ; if file_path . exists () { return file_path ; } if self_contained { let file_path = sess . target_tlib_path . dir . join ("self-contained") . join (name) ; if file_path . exists () { return file_path ; } } for search_path in sess . target_filesearch () . search_paths (PathKind :: Native) { let file_path = search_path . dir . join (name) ; if file_path . exists () { return file_path ; } } PathBuf :: from (name) }
    };
}

get_object_file_path!()