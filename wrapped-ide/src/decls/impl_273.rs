macro_rules! deps {
    () => {
        InlayHintLabel!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl fmt :: Display for InlayHintLabel { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . parts . iter () . map (| part | & part . text) . format ("")) } }
    };
}

impl_273!()