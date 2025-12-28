macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < I , F > Iterator for Update < I , F > where I : Iterator , F : FnMut (& mut I :: Item) , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { if let Some (mut v) = self . iter . next () { (self . f) (& mut v) ; Some (v) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } fn fold < Acc , G > (self , init : Acc , mut g : G) -> Acc where G : FnMut (Acc , Self :: Item) -> Acc , { let mut f = self . f ; self . iter . fold (init , move | acc , mut v | { f (& mut v) ; g (acc , v) }) } fn collect < C > (self) -> C where C : FromIterator < Self :: Item > , { let mut f = self . f ; self . iter . map (move | mut v | { f (& mut v) ; v }) . collect () } }
    };
}

impl_142!()