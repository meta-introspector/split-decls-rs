macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl Ord for File { fn cmp (& self , other : & Self) -> Ordering { self . bytes . as_ptr () . cmp (& other . bytes . as_ptr ()) } }
    };
}

impl_438!();