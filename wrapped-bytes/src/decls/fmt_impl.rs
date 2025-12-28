macro_rules! deps {
    () => {
        BytesRef!();
    };
}

macro_rules! fmt_impl {
    () => {
        deps!();
        macro_rules ! fmt_impl { ($ tr : ident , $ ty : ty) => { impl $ tr for $ ty { fn fmt (& self , f : & mut Formatter <'_ >) -> Result { $ tr :: fmt (& BytesRef (self . as_ref ()) , f) } } } ; }
    };
}

fmt_impl!();