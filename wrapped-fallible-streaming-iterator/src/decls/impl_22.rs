macro_rules! deps {
    () => {
        FallibleStreamingIterator!();
        MapRef!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < I , F , B : ? Sized > FallibleStreamingIterator for MapRef < I , F > where I : FallibleStreamingIterator , F : Fn (& I :: Item) -> & B , { type Item = B ; type Error = I :: Error ; # [inline] fn advance (& mut self) -> Result < () , I :: Error > { self . it . advance () } # [inline] fn get (& self) -> Option < & B > { self . it . get () . map (& self . f) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_22!();