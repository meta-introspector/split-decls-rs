macro_rules! deps {
    () => {
        ExtractIf!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl < K , F , A : Allocator > Iterator for ExtractIf < '_ , K , F , A > where F : FnMut (& K) -> bool , { type Item = K ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < Self :: Item > { self . inner . next (| & mut (ref k , ()) | (self . f) (k)) . map (| (k , ()) | k) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . inner . iter . size_hint () . 1) } }
    };
}

impl_438!()