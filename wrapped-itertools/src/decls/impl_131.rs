macro_rules! deps {
    () => {
        FilterMapOk!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < I , F , T , U , E > DoubleEndedIterator for FilterMapOk < I , F > where I : DoubleEndedIterator < Item = Result < T , E > > , F : FnMut (T) -> Option < U > , { fn next_back (& mut self) -> Option < Self :: Item > { let f = & mut self . f ; self . iter . by_ref () . rev () . find_map (| res | match res { Ok (t) => f (t) . map (Ok) , Err (e) => Some (Err (e)) , }) } fn rfold < Acc , Fold > (self , init : Acc , fold_f : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { let mut f = self . f ; self . iter . filter_map (| v | transpose_result (v . map (& mut f))) . rfold (init , fold_f) } }
    };
}

impl_131!();