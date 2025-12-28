macro_rules! deps {
    () => {
        Bool!();
    };
}

macro_rules! FALSE {
    () => {
        deps!();
        pub (crate) const FALSE : Bool = Bool :: FALSE ;
    };
}

FALSE!()