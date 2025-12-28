macro_rules! deps {
    () => {
        CrateType!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl From < & str > for CrateType { fn from (value : & str) -> Self { match value { "bin" => CrateType :: Bin , "lib" => CrateType :: Lib , "rlib" => CrateType :: RLib , "dylib" => CrateType :: DyLib , "cdylib" => CrateType :: CDyLib , "staticlib" => CrateType :: StaticLib , "proc-macro" => CrateType :: ProcMacro , x => CrateType :: Unknown (x . to_string ()) , } } }
    };
}

impl_36!()