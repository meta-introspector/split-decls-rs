macro_rules! deps {
    () => {
        FilterOk!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < I , F , T , E > DoubleEndedIterator for FilterOk < I , F > where I : DoubleEndedIterator < Item = Result < T , E > > , F : FnMut (& T) -> bool , { fn next_back (& mut self) -> Option < Self :: Item > { let f = & mut self . f ; self . iter . rfind (| res | match res { Ok (t) => f (t) , _ => true , }) } fn rfold < Acc , Fold > (self , init : Acc , fold_f : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { let mut f = self . f ; self . iter . filter (| v | v . as_ref () . map (& mut f) . unwrap_or (true)) . rfold (init , fold_f) } }
    };
}

impl_124!()