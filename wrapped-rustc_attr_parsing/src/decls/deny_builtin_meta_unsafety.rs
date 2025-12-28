macro_rules! deps {
    () => {
        InvalidAttrUnsafe!();
    };
}

macro_rules! deny_builtin_meta_unsafety {
    () => {
        deps!();
        pub fn deny_builtin_meta_unsafety (diag : DiagCtxtHandle < '_ > , unsafety : Safety , name : & Path) { if let Safety :: Unsafe (unsafe_span) = unsafety { diag . emit_err (errors :: InvalidAttrUnsafe { span : unsafe_span , name : name . clone () }) ; } }
    };
}

deny_builtin_meta_unsafety!();