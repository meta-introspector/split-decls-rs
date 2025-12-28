macro_rules! deps {
    () => {
        CollectConsumer!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl < T : Send > CollectConsumer < '_ , T > { # [doc = " Create a collector for `len` items in the unused capacity of the vector."] pub (super) fn appender (vec : & mut Vec < T > , len : usize) -> CollectConsumer < '_ , T > { let start = vec . len () ; assert ! (vec . capacity () - start >= len) ; unsafe { CollectConsumer :: new (vec . as_mut_ptr () . add (start) , len) } } }
    };
}

impl_341!()