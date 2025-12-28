macro_rules! deps {
    () => {
        Array!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T > Array < T > { fn layout (len : usize) -> Layout { Layout :: new :: < Self > () . extend (Layout :: array :: < MaybeUninit < T > > (len) . unwrap ()) . unwrap () . 0 . pad_to_align () } }
    };
}

impl_16!();