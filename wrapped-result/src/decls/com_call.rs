macro_rules! com_call {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! com_call { ($ vtbl : ty , $ this : ident .$ method : ident ($ ($ args : tt) *)) => { ((&** ($ this . as_raw () as * mut * mut $ vtbl)) .$ method) ($ this . as_raw () , $ ($ args) *) } }
    };
}

com_call!();