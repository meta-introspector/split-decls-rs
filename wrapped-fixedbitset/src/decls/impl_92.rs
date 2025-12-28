macro_rules! deps {
    () => {
        Difference!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < 'a > Iterator for Difference < 'a > { type Item = usize ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . iter . by_ref () . find (| & nxt | ! self . other . contains (nxt)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_92!();