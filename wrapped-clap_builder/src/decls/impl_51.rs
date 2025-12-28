macro_rules! deps {
    () => {
        Arg!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl PartialEq for Arg { fn eq (& self , other : & Arg) -> bool { self . get_id () == other . get_id () } }
    };
}

impl_51!();