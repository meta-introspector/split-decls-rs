macro_rules! root {
    () => {
        # [doc = " Path to the test's filesystem scratchpad"] # [doc = ""] # [doc = " ex: `$CARGO_TARGET_TMPDIR/cit/t0`"] pub fn root () -> PathBuf { let id = TEST_ID . with (| n | { n . borrow () . expect ("Tests must use the `#[cargo_test]` attribute in \
             order to be able to use the crate root." ,) }) ; let mut root = global_root () ; root . push (& format ! ("t{}" , id)) ; root }
    };
}

root!();