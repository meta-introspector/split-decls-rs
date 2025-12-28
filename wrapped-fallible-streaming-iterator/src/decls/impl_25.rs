macro_rules! deps {
    () => {
        FallibleStreamingIterator!();
        MapErr!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < I , F , B > FallibleStreamingIterator for MapErr < I , F > where I : FallibleStreamingIterator , F : Fn (I :: Error) -> B , { type Item = I :: Item ; type Error = B ; # [inline] fn advance (& mut self) -> Result < () , B > { self . it . advance () . map_err (& mut self . f) } # [inline] fn get (& self) -> Option < & I :: Item > { self . it . get () } # [inline] fn next (& mut self) -> Result < Option < & I :: Item > , B > { self . it . next () . map_err (& mut self . f) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_25!();