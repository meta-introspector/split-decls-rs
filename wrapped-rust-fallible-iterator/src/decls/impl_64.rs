macro_rules! deps {
    () => {
        FallibleIterator!();
        Cloned!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < 'a , T , I > FallibleIterator for Cloned < I > where I : FallibleIterator < Item = & 'a T > , T : 'a + Clone , { type Item = T ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < T > , I :: Error > { self . 0 . next () . map (| o | o . cloned ()) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } # [inline] fn try_fold < B , E , F > (& mut self , init : B , mut f : F) -> Result < B , E > where E : From < I :: Error > , F : FnMut (B , T) -> Result < B , E > , { self . 0 . try_fold (init , | acc , v | f (acc , v . clone ())) } }
    };
}

impl_64!();