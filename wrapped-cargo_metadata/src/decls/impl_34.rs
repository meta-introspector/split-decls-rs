macro_rules! deps {
    () => {
        TargetKind!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl fmt :: Display for TargetKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Bench => "bench" . fmt (f) , Self :: Bin => "bin" . fmt (f) , Self :: CustomBuild => "custom-build" . fmt (f) , Self :: CDyLib => "cdylib" . fmt (f) , Self :: DyLib => "dylib" . fmt (f) , Self :: Example => "example" . fmt (f) , Self :: Lib => "lib" . fmt (f) , Self :: ProcMacro => "proc-macro" . fmt (f) , Self :: RLib => "rlib" . fmt (f) , Self :: StaticLib => "staticlib" . fmt (f) , Self :: Test => "test" . fmt (f) , Self :: Unknown (x) => x . fmt (f) , } } }
    };
}

impl_34!()