macro_rules! deps {
    () => {
        IdentFragment!();
    };
}

macro_rules! ident_fragment_display {
    () => {
        deps!();
        macro_rules ! ident_fragment_display { ($ ($ T : ty) ,*) => { $ (impl IdentFragment for $ T { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (self , f) } }) * } ; }
    };
}

ident_fragment_display!()