macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_572 {
    () => {
        deps!();
        impl fmt :: Debug for State { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let rs = self . transitions . iter () . map (| t | format ! ("{t:?}")) . collect :: < Vec < String > > () . join (", ") ; write ! (f , "{rs}") } }
    };
}

impl_572!();