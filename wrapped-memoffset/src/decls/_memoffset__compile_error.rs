macro_rules! _memoffset__compile_error {
    () => {
        # [doc = " Reexport for `local_inner_macros`; see"] # [doc = " <https://doc.rust-lang.org/edition-guide/rust-2018/macros/macro-changes.html#macros-using-local_inner_macros>."] # [doc (hidden)] # [macro_export] macro_rules ! _memoffset__compile_error { ($ ($ inner : tt) *) => { compile_error ! { $ ($ inner) * } } }
    };
}

_memoffset__compile_error!();