macro_rules! Shared {
    () => {
        struct Shared { vec : Vec < u8 > , original_capacity_repr : usize , ref_count : AtomicUsize , }
    };
}

Shared!()