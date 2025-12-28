macro_rules! deps {
    () => {
        MapSpecialCaseFn!();
        MapSpecialCase!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < I , R > Iterator for MapSpecialCase < I , R > where I : Iterator , R : MapSpecialCaseFn < I :: Item > , { type Item = R :: Out ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| i | self . f . call (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } fn fold < Acc , Fold > (self , init : Acc , mut fold_f : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { let mut f = self . f ; self . iter . fold (init , move | acc , v | fold_f (acc , f . call (v))) } fn collect < C > (self) -> C where C : FromIterator < Self :: Item > , { let mut f = self . f ; self . iter . map (move | v | f . call (v)) . collect () } }
    };
}

impl_45!()