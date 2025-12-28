macro_rules! deps {
    () => {
        Project!();
    };
}

macro_rules! test_path_macro {
    () => {
        deps!();
        # [test] fn test_path_macro () { use std :: path :: { Path , PathBuf } ; struct Project { dir : PathBuf , } let project = Project { dir : PathBuf :: from ("../target/tests") , } ; let cargo_dir = path ! (project . dir / ".cargo" / "config.toml") ; assert_eq ! (cargo_dir , Path :: new ("../target/tests/.cargo/config.toml")) ; }
    };
}

test_path_macro!()