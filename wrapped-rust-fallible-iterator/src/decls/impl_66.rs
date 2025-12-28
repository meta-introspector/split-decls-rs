macro_rules! deps {
    () => {
        DoubleEndedFallibleIterator!();
        FallibleIterator!();
        Rev!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < I > FallibleIterator for Rev < I > where I : DoubleEndedFallibleIterator , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , I :: Error > { self . 0 . next_back () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } # [inline] fn count (self) -> Result < usize , I :: Error > { self . 0 . count () } # [inline] fn try_fold < B , E , F > (& mut self , init : B , f : F) -> Result < B , E > where E : From < I :: Error > , F : FnMut (B , I :: Item) -> Result < B , E > , { self . 0 . try_rfold (init , f) } }
    };
}

impl_66!()