macro_rules! remove_file_with_permission_check {
    () => {
        fn remove_file_with_permission_check (p : & Path) -> Result < () > { remove_with_permission_check (fs :: remove_file , p) . with_context (| | format ! ("failed to remove file `{}`" , p . display ())) }
    };
}

remove_file_with_permission_check!()