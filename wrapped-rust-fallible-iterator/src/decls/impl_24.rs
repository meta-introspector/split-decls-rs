macro_rules! deps {
    () => {
        DoubleEndedFallibleIterator!();
        Cloned!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < 'a , T , I > DoubleEndedFallibleIterator for Cloned < I > where I : DoubleEndedFallibleIterator < Item = & 'a T > , T : 'a + Clone , { # [inline] fn next_back (& mut self) -> Result < Option < T > , I :: Error > { self . 0 . next_back () . map (| o | o . cloned ()) } # [inline] fn try_rfold < B , E , F > (& mut self , init : B , mut f : F) -> Result < B , E > where E : From < I :: Error > , F : FnMut (B , T) -> Result < B , E > , { self . 0 . try_rfold (init , | acc , v | f (acc , v . clone ())) } }
    };
}

impl_24!()