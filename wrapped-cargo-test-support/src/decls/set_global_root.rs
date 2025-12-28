macro_rules! set_global_root {
    () => {
        fn set_global_root (tmp_dir : & 'static str) { let mut lock = GLOBAL_ROOT . get_or_init (| | Default :: default ()) . lock () . unwrap () ; if lock . is_none () { let mut root = PathBuf :: from (tmp_dir) ; root . push (CARGO_INTEGRATION_TEST_DIR) ; * lock = Some (root) ; } }
    };
}

set_global_root!()