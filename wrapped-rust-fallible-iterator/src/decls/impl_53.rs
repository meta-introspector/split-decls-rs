macro_rules! deps {
    () => {
        DoubleEndedFallibleIterator!();
        Inspect!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < I , F > DoubleEndedFallibleIterator for Inspect < I , F > where I : DoubleEndedFallibleIterator , F : FnMut (& I :: Item) -> Result < () , I :: Error > , { # [inline] fn next_back (& mut self) -> Result < Option < I :: Item > , I :: Error > { match self . it . next_back () ? { Some (i) => { (self . f) (& i) ? ; Ok (Some (i)) } None => Ok (None) , } } # [inline] fn try_rfold < B , E , G > (& mut self , init : B , mut f : G) -> Result < B , E > where E : From < I :: Error > , G : FnMut (B , I :: Item) -> Result < B , E > , { let inspect = & mut self . f ; self . it . try_rfold (init , | acc , v | { inspect (& v) ? ; f (acc , v) }) } }
    };
}

impl_53!()