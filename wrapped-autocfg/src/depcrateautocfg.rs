// Generated macro for AutoCfg (struct)
macro_rules! DepcrateAutoCfg {
() => {
// Module: crate
// Provides: {"AutoCfg"}
// Dependencies: {}
# [doc = " Helper to detect compiler features for `cfg` output in build scripts."] # [derive (Clone , Debug)] pub struct AutoCfg { out_dir : PathBuf , rustc : Rustc , rustc_version : Version , target : Option < OsString > , no_std : bool , edition : Option < String > , rustflags : Vec < String > , uuid : u64 , }
};
}
