macro_rules! deps {
    () => {
        CrateType!();
        Result!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl fmt :: Display for CrateType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Bin => "bin" . fmt (f) , Self :: CDyLib => "cdylib" . fmt (f) , Self :: DyLib => "dylib" . fmt (f) , Self :: Lib => "lib" . fmt (f) , Self :: ProcMacro => "proc-macro" . fmt (f) , Self :: RLib => "rlib" . fmt (f) , Self :: StaticLib => "staticlib" . fmt (f) , Self :: Unknown (x) => x . fmt (f) , } } }
    };
}

impl_73!();