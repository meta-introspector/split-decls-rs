macro_rules! deps {
    () => {
        Version!();
        Rustc!();
    };
}

macro_rules! AutoCfg {
    () => {
        deps!();
        # [doc = " Helper to detect compiler features for `cfg` output in build scripts."] # [derive (Clone , Debug)] pub struct AutoCfg { out_dir : PathBuf , rustc : Rustc , rustc_version : Version , target : Option < OsString > , no_std : bool , edition : Option < String > , rustflags : Vec < String > , uuid : u64 , }
    };
}

AutoCfg!()