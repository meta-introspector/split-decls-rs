macro_rules! deps {
    () => {
        FallibleIterator!();
        FoldStop!();
        FilterMap!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < B , I , F > FallibleIterator for FilterMap < I , F > where I : FallibleIterator , F : FnMut (I :: Item) -> Result < Option < B > , I :: Error > , { type Item = B ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < B > , I :: Error > { let map = & mut self . f ; self . it . try_fold (() , | () , v | match map (v) ? { Some (v) => Err (FoldStop :: Break (Some (v))) , None => Ok (()) , }) . map (| () | None) . unpack_fold () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . it . size_hint () . 1) } # [inline] fn try_fold < C , E , G > (& mut self , init : C , mut f : G) -> Result < C , E > where E : From < I :: Error > , G : FnMut (C , B) -> Result < C , E > , { let map = & mut self . f ; self . it . try_fold (init , | acc , v | match map (v) ? { Some (v) => f (acc , v) , None => Ok (acc) , }) } }
    };
}

impl_80!();