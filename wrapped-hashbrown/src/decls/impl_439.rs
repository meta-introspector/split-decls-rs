macro_rules! deps {
    () => {
        ExtractIf!();
    };
}

macro_rules! impl_439 {
    () => {
        deps!();
        impl < K , F , A : Allocator > FusedIterator for ExtractIf < '_ , K , F , A > where F : FnMut (& K) -> bool { }
    };
}

impl_439!()