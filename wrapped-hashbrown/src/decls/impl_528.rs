macro_rules! deps {
    () => {
        ExtractIf!();
    };
}

macro_rules! impl_528 {
    () => {
        deps!();
        impl < T , F , A : Allocator > FusedIterator for ExtractIf < '_ , T , F , A > where F : FnMut (& mut T) -> bool { }
    };
}

impl_528!()