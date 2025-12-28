macro_rules! debug_fmt_fields {
    () => {
        macro_rules ! debug_fmt_fields { ($ tyname : ident , $ ($ ($ field : tt) .+) ,*) => { fn fmt (& self , f : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { f . debug_struct (stringify ! ($ tyname)) $ (. field (stringify ! ($ ($ field) .+) , & self .$ ($ field) .+)) * . finish () } } }
    };
}

debug_fmt_fields!()