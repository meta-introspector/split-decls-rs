macro_rules! deps {
    () => {
        DoubleEndedFallibleIterator!();
        IntoFallible!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T , I > DoubleEndedFallibleIterator for IntoFallible < I > where I : DoubleEndedIterator < Item = T > , { # [inline] fn next_back (& mut self) -> Result < Option < T > , Infallible > { Ok (self . 0 . next_back ()) } # [inline] fn try_rfold < B , E2 , F > (& mut self , init : B , f : F) -> Result < B , E2 > where E2 : From < Infallible > , F : FnMut (B , T) -> Result < B , E2 > , { self . 0 . try_rfold (init , f) } }
    };
}

impl_32!()