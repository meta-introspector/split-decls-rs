macro_rules! deps {
    () => {
        InValues!();
        Filters!();
        Result!();
    };
}

macro_rules! impl_569 {
    () => {
        deps!();
        # [cfg (feature = "modern_sqlite")] impl < 'a > Filters < 'a > { # [doc = " Find all elements on the right-hand side of an IN constraint"] pub fn in_values (& self , idx : usize) -> Result < InValues < '_ > > { let list = self . args [idx] ; Ok (InValues { list , phantom : PhantomData , first : true , }) } }
    };
}

impl_569!();