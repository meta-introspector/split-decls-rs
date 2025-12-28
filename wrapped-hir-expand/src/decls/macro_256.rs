macro_rules! deps {
    () => {
        ModPath!();
        AttrInput!();
    };
}

macro_rules! macro_256 {
    () => {
        deps!();
        intern :: impl_internable ! (ModPath , attrs :: AttrInput) ;
    };
}

macro_256!();