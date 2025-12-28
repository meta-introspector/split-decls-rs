macro_rules! deps {
    () => {
        Align16!();
        AtomicMaybeUninit!();
        Primitive!();
    };
}

macro_rules! Array {
    () => {
        deps!();
        pub (crate) struct Array < T : Primitive > { arr : Box < Align16 < [AtomicMaybeUninit < T > ; 10] > > , base : T , idx : usize , }
    };
}

Array!()