macro_rules! mode_is_dir {
    () => {
        fn mode_is_dir (mode : Option < gix_index :: entry :: Mode >) -> Option < bool > { mode . map (| m | m . is_sparse () || m . is_submodule ()) }
    };
}

mode_is_dir!()