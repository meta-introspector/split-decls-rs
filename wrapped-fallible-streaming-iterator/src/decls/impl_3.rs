macro_rules! deps {
    () => {
        FallibleStreamingIterator!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < 'a , I : ? Sized > FallibleStreamingIterator for & 'a mut I where I : FallibleStreamingIterator , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn advance (& mut self) -> Result < () , I :: Error > { (* * self) . advance () } # [inline] fn get (& self) -> Option < & I :: Item > { (* * self) . get () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (* * self) . size_hint () } # [inline] fn next (& mut self) -> Result < Option < & I :: Item > , I :: Error > { (* * self) . next () } }
    };
}

impl_3!()