macro_rules! deps {
    () => {
        Error!();
        Result!();
        UnitRef!();
    };
}

macro_rules! render_file {
    () => {
        deps!();
        fn render_file < R : gimli :: Reader > (dw_unit : gimli :: UnitRef < R > , file : & gimli :: FileEntry < R , R :: Offset > , header : & gimli :: LineProgramHeader < R , R :: Offset > ,) -> Result < String , gimli :: Error > { let mut path = if let Some (ref comp_dir) = dw_unit . comp_dir { comp_dir . to_string_lossy () ? . into_owned () } else { String :: new () } ; if file . directory_index () != 0 { if let Some (directory) = file . directory (header) { path_push (& mut path , dw_unit . attr_string (directory) ? . to_string_lossy () ? . as_ref () ,) ; } } path_push (& mut path , dw_unit . attr_string (file . path_name ()) ? . to_string_lossy () ? . as_ref () ,) ; Ok (path) }
    };
}

render_file!();