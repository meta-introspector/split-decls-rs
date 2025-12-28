macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        # [doc = " Prints a span in a form convenient for debugging."] impl Debug for Span { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . inner , f) } }
    };
}

impl_205!()