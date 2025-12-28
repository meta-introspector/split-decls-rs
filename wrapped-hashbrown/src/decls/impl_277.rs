macro_rules! deps {
    () => {
        ExtractIf!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl < K , V , F > FusedIterator for ExtractIf < '_ , K , V , F > where F : FnMut (& K , & mut V) -> bool { }
    };
}

impl_277!();