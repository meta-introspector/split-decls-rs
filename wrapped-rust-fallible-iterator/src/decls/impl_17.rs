macro_rules! deps {
    () => {
        DoubleEndedFallibleIterator!();
        Map!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < B , F , I > DoubleEndedFallibleIterator for Map < I , F > where I : DoubleEndedFallibleIterator , F : FnMut (I :: Item) -> Result < B , I :: Error > , { # [inline] fn next_back (& mut self) -> Result < Option < B > , I :: Error > { match self . it . next_back () { Ok (Some (v)) => Ok (Some ((self . f) (v) ?)) , Ok (None) => Ok (None) , Err (e) => Err (e) , } } # [inline] fn try_rfold < C , E , G > (& mut self , init : C , mut f : G) -> Result < C , E > where E : From < I :: Error > , G : FnMut (C , B) -> Result < C , E > , { let map = & mut self . f ; self . it . try_rfold (init , | acc , v | f (acc , map (v) ?)) } }
    };
}

impl_17!()