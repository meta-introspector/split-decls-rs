macro_rules! deps {
    () => {
        PeekNth!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        impl < I > Iterator for PeekNth < I > where I : Iterator , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . buf . pop_front () . or_else (| | self . iter . next ()) } fn size_hint (& self) -> (usize , Option < usize >) { size_hint :: add_scalar (self . iter . size_hint () , self . buf . len ()) } fn fold < B , F > (self , mut init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { init = self . buf . into_iter () . fold (init , & mut f) ; self . iter . fold (init , f) } }
    };
}

impl_395!()