macro_rules! deps {
    () => {
        MapRef!();
        DoubleEndedFallibleStreamingIterator!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < I , F , B : ? Sized > DoubleEndedFallibleStreamingIterator for MapRef < I , F > where I : DoubleEndedFallibleStreamingIterator , F : Fn (& I :: Item) -> & B , { # [inline] fn advance_back (& mut self) -> Result < () , I :: Error > { self . it . advance_back () } }
    };
}

impl_23!();