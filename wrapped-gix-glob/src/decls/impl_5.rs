macro_rules! deps {
    () => {
        Pattern!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl fmt :: Display for Pattern { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . mode . contains (Mode :: NEGATIVE) { "!" . fmt (f) ? ; } if self . mode . contains (Mode :: ABSOLUTE) { "/" . fmt (f) ? ; } self . text . fmt (f) ? ; if self . mode . contains (Mode :: MUST_BE_DIR) { "/" . fmt (f) ? ; } Ok (()) } }
    };
}

impl_5!();