macro_rules! deps {
    () => {
        LangCrateOrigin!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl fmt :: Display for LangCrateOrigin { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let text = match self { LangCrateOrigin :: Alloc => "alloc" , LangCrateOrigin :: Core => "core" , LangCrateOrigin :: ProcMacro => "proc_macro" , LangCrateOrigin :: Std => "std" , LangCrateOrigin :: Test => "test" , LangCrateOrigin :: Other => "other" , } ; f . write_str (text) } }
    };
}

impl_27!()