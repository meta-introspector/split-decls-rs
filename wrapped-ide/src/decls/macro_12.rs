macro_rules! deps {
    () => {
        Markup!();
    };
}

macro_rules! macro_12 {
    () => {
        deps!();
        impl_empty_upmap_from_ra_fixture ! (Markup) ;
    };
}

macro_12!();