macro_rules! deps {
    () => {
        DoubleEndedFallibleStreamingIterator!();
        MapErr!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < I , F , B > DoubleEndedFallibleStreamingIterator for MapErr < I , F > where I : DoubleEndedFallibleStreamingIterator , F : Fn (I :: Error) -> B , { # [inline] fn advance_back (& mut self) -> Result < () , B > { self . it . advance_back () . map_err (& mut self . f) } # [inline] fn next_back (& mut self) -> Result < Option < & I :: Item > , B > { self . it . next_back () . map_err (& mut self . f) } }
    };
}

impl_26!()