macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < T > Symbol < Option < T > > { # [doc = " Lift Option out of the symbol."] pub fn lift_option (self) -> Option < Symbol < T > > { if self . pointer . is_null () { None } else { Some (Symbol { pointer : self . pointer , pd : marker :: PhantomData , }) } } }
    };
}

impl_97!()