macro_rules! deps {
    () => {
        Binding!();
    };
}

macro_rules! BindingId {
    () => {
        deps!();
        pub type BindingId = Idx < Binding > ;
    };
}

BindingId!()