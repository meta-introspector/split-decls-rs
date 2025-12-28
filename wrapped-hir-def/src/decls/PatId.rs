macro_rules! deps {
    () => {
        Pat!();
    };
}

macro_rules! PatId {
    () => {
        deps!();
        pub type PatId = Idx < Pat > ;
    };
}

PatId!()