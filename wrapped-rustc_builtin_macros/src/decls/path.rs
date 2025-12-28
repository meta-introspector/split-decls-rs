macro_rules! path {
    () => {
        macro_rules ! path { ($ span : expr , $ ($ part : ident) ::*) => { vec ! [$ (Ident :: new (sym ::$ part , $ span) ,) *] } }
    };
}

path!()