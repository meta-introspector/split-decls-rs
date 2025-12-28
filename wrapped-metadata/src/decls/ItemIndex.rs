macro_rules! deps {
    () => {
        HashType!();
    };
}

macro_rules! ItemIndex {
    () => {
        deps!();
        pub struct ItemIndex < 'a > (HashType < 'a >) ;
    };
}

ItemIndex!();