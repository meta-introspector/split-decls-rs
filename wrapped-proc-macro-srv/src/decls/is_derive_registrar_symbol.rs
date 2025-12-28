macro_rules! is_derive_registrar_symbol {
    () => {
        fn is_derive_registrar_symbol (symbol : & str) -> bool { const NEW_REGISTRAR_SYMBOL : & str = "_rustc_proc_macro_decls_" ; symbol . contains (NEW_REGISTRAR_SYMBOL) }
    };
}

is_derive_registrar_symbol!();