macro_rules! project_root {
    () => {
        pub fn project_root () -> PathBuf { let dir = env ! ("CARGO_MANIFEST_DIR") ; let mut res = PathBuf :: from (dir) ; while ! res . join ("README.md") . exists () { res = res . parent () . expect ("reached fs root without finding project root") . to_owned () } res }
    };
}

project_root!();