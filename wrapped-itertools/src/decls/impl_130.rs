macro_rules! deps {
    () => {
        FilterMapOk!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < I , F , T , U , E > Iterator for FilterMapOk < I , F > where I : Iterator < Item = Result < T , E > > , F : FnMut (T) -> Option < U > , { type Item = Result < U , E > ; fn next (& mut self) -> Option < Self :: Item > { let f = & mut self . f ; self . iter . find_map (| res | match res { Ok (t) => f (t) . map (Ok) , Err (e) => Some (Err (e)) , }) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . iter . size_hint () . 1) } fn fold < Acc , Fold > (self , init : Acc , fold_f : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { let mut f = self . f ; self . iter . filter_map (| v | transpose_result (v . map (& mut f))) . fold (init , fold_f) } fn collect < C > (self) -> C where C : FromIterator < Self :: Item > , { let mut f = self . f ; self . iter . filter_map (| v | transpose_result (v . map (& mut f))) . collect () } }
    };
}

impl_130!()