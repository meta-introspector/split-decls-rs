macro_rules! deps {
    () => {
        PutBackN!();
    };
}

macro_rules! impl_454 {
    () => {
        deps!();
        impl < I : Iterator > Iterator for PutBackN < I > { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . top . pop () . or_else (| | self . iter . next ()) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { size_hint :: add_scalar (self . iter . size_hint () , self . top . len ()) } fn fold < B , F > (self , mut init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { init = self . top . into_iter () . rfold (init , & mut f) ; self . iter . fold (init , f) } }
    };
}

impl_454!();