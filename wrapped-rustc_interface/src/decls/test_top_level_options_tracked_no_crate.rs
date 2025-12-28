macro_rules! test_top_level_options_tracked_no_crate {
    () => {
        # [test] fn test_top_level_options_tracked_no_crate () { let reference = Options :: default () ; let mut opts ; macro_rules ! tracked { ($ name : ident , $ non_default_value : expr) => { opts = reference . clone () ; assert_ne ! (opts .$ name , $ non_default_value) ; opts .$ name = $ non_default_value ; assert_eq ! (reference . dep_tracking_hash (true) , opts . dep_tracking_hash (true)) ; assert_ne ! (reference . dep_tracking_hash (false) , opts . dep_tracking_hash (false)) ; } ; } tracked ! (real_rust_source_base_dir , Some ("/home/bors/rust/.rustup/toolchains/nightly/lib/rustlib/src/rust" . into ())) ; tracked ! (remap_path_prefix , vec ! [("/home/bors/rust" . into () , "src" . into ())]) ; }
    };
}

test_top_level_options_tracked_no_crate!()