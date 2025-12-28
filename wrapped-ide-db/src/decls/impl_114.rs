macro_rules! deps {
    () => {
        IsEmpty!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < T , const N : usize > IsEmpty for SmallVec < [T ; N] > { fn is_empty (& self) -> bool { self . is_empty () } }
    };
}

impl_114!();