macro_rules! Tool {
    () => {
        # [doc = " `Tool` found by `find-msvc-tools`"] # [derive (Clone , Debug)] pub struct Tool { pub (crate) tool : PathBuf , pub (crate) is_clang_cl : bool , pub (crate) env : Vec < (OsString , OsString) > , }
    };
}

Tool!();