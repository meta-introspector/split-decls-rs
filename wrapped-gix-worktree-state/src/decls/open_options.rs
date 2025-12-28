macro_rules! open_options {
    () => {
        fn open_options (path : & Path , destination_is_initially_empty : bool , overwrite_existing : bool) -> std :: fs :: OpenOptions { if overwrite_existing || ! destination_is_initially_empty { debug_assert_dest_is_no_symlink (path) ; } let mut options = gix_features :: fs :: open_options_no_follow () ; options . create_new (destination_is_initially_empty && ! overwrite_existing) . create (! destination_is_initially_empty || overwrite_existing) . write (true) . truncate (true) ; options }
    };
}

open_options!();