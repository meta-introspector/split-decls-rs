macro_rules! deps {
    () => {
        DoubleEndedFallibleIterator!();
        FoldStop!();
        FilterMap!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < B , I , F > DoubleEndedFallibleIterator for FilterMap < I , F > where I : DoubleEndedFallibleIterator , F : FnMut (I :: Item) -> Result < Option < B > , I :: Error > , { # [inline] fn next_back (& mut self) -> Result < Option < B > , I :: Error > { let map = & mut self . f ; self . it . try_rfold (() , | () , v | match map (v) ? { Some (v) => Err (FoldStop :: Break (Some (v))) , None => Ok (()) , }) . map (| () | None) . unpack_fold () } # [inline] fn try_rfold < C , E , G > (& mut self , init : C , mut f : G) -> Result < C , E > where E : From < I :: Error > , G : FnMut (C , B) -> Result < C , E > , { let map = & mut self . f ; self . it . try_rfold (init , | acc , v | match map (v) ? { Some (v) => f (acc , v) , None => Ok (acc) , }) } }
    };
}

impl_81!();