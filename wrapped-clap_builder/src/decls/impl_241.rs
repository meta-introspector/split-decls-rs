macro_rules! deps {
    () => {
        StyledStr!();
        Result!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        # [doc = " Color-unaware printing. Never uses coloring."] impl std :: fmt :: Display for StyledStr { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { for part in self . iter_text () { part . fmt (f) ? ; } Ok (()) } }
    };
}

impl_241!()