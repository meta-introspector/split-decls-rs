macro_rules! deps {
    () => {
        FoldStop!();
        FallibleIterator!();
        Filter!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < I , F > FallibleIterator for Filter < I , F > where I : FallibleIterator , F : FnMut (& I :: Item) -> Result < bool , I :: Error > , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , I :: Error > { let filter = & mut self . f ; self . it . try_fold (() , | () , v | { if filter (& v) ? { return Err (FoldStop :: Break (Some (v))) ; } Ok (()) }) . map (| () | None) . unpack_fold () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . it . size_hint () . 1) } # [inline] fn try_fold < B , E , G > (& mut self , init : B , mut f : G) -> Result < B , E > where E : From < I :: Error > , G : FnMut (B , I :: Item) -> Result < B , E > , { let predicate = & mut self . f ; self . it . try_fold (init , | acc , v | { if predicate (& v) ? { f (acc , v) } else { Ok (acc) } } ,) } }
    };
}

impl_77!();