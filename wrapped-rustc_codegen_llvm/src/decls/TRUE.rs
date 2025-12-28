macro_rules! deps {
    () => {
        Bool!();
    };
}

macro_rules! TRUE {
    () => {
        deps!();
        pub (crate) const TRUE : Bool = Bool :: TRUE ;
    };
}

TRUE!()