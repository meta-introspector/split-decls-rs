macro_rules! global_root {
    () => {
        # [doc = " Path to the parent directory of all test [`root`]s"] # [doc = ""] # [doc = " ex: `$CARGO_TARGET_TMPDIR/cit`"] pub fn global_root () -> PathBuf { let lock = GLOBAL_ROOT . get_or_init (| | Default :: default ()) . lock () . unwrap () ; match lock . as_ref () { Some (p) => p . clone () , None => unreachable ! ("GLOBAL_ROOT not set yet") , } }
    };
}

global_root!()