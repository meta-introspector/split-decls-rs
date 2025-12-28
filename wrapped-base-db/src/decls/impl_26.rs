macro_rules! deps {
    () => {
        LangCrateOrigin!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl From < & str > for LangCrateOrigin { fn from (s : & str) -> Self { match s { "alloc" => LangCrateOrigin :: Alloc , "core" => LangCrateOrigin :: Core , "proc-macro" | "proc_macro" => LangCrateOrigin :: ProcMacro , "std" => LangCrateOrigin :: Std , "test" => LangCrateOrigin :: Test , _ => LangCrateOrigin :: Other , } } }
    };
}

impl_26!()