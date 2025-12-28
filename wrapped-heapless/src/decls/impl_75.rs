macro_rules! deps {
    () => {
        OldestOrdered!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < 'a , T > Iterator for OldestOrdered < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_75!()