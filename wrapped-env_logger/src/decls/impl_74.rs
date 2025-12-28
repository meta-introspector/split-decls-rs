macro_rules! deps {
    () => {
        Formatter!();
        StyledValue!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        # [cfg (feature = "color")] impl < T : Display > Display for StyledValue < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let style = self . style ; write ! (f , "{style}") ? ; self . value . fmt (f) ? ; write ! (f , "{style:#}") ? ; Ok (()) } }
    };
}

impl_74!();