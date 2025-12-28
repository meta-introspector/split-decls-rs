macro_rules! deps {
    () => {
        ToDef!();
        SemanticsImpl!();
    };
}

macro_rules! to_def_impls {
    () => {
        deps!();
        macro_rules ! to_def_impls { ($ (($ def : path , $ ast : path , $ meth : ident)) ,* ,) => { $ (impl ToDef for $ ast { type Def = $ def ; fn to_def (sema : & SemanticsImpl <'_ >, src : InFile <& Self >) -> Option < Self :: Def > { sema . with_ctx (| ctx | ctx .$ meth (src)) . map (<$ def >:: from) } }) * } }
    };
}

to_def_impls!()