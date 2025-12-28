macro_rules! deps {
    () => {
        UpdateSeq!();
    };
}

macro_rules! impl_983 {
    () => {
        deps!();
        impl < I , F > Iterator for UpdateSeq < I , F > where I : Iterator , F : Fn (& mut I :: Item) , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { let mut v = self . base . next () ? ; (self . update_op) (& mut v) ; Some (v) } fn size_hint (& self) -> (usize , Option < usize >) { self . base . size_hint () } fn fold < Acc , G > (self , init : Acc , g : G) -> Acc where G : FnMut (Acc , Self :: Item) -> Acc , { self . base . map (apply (self . update_op)) . fold (init , g) } fn collect < C > (self) -> C where C : :: std :: iter :: FromIterator < Self :: Item > , { self . base . map (apply (self . update_op)) . collect () } }
    };
}

impl_983!()