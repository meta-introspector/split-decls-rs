macro_rules! deps {
    () => {
        DoubleEndedFallibleIterator!();
        Rev!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < I > DoubleEndedFallibleIterator for Rev < I > where I : DoubleEndedFallibleIterator , { # [inline] fn next_back (& mut self) -> Result < Option < I :: Item > , I :: Error > { self . 0 . next () } # [inline] fn try_rfold < B , E , F > (& mut self , init : B , f : F) -> Result < B , E > where E : From < I :: Error > , F : FnMut (B , I :: Item) -> Result < B , E > , { self . 0 . try_fold (init , f) } }
    };
}

impl_108!();