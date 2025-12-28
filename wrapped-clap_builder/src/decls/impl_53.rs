macro_rules! deps {
    () => {
        Arg!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl Ord for Arg { fn cmp (& self , other : & Arg) -> Ordering { self . get_id () . cmp (other . get_id ()) } }
    };
}

impl_53!()