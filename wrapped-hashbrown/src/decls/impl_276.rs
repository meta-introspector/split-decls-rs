macro_rules! deps {
    () => {
        ExtractIf!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < K , V , F , A > Iterator for ExtractIf < '_ , K , V , F , A > where F : FnMut (& K , & mut V) -> bool , A : Allocator , { type Item = (K , V) ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < Self :: Item > { self . inner . next (| & mut (ref k , ref mut v) | (self . f) (k , v)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . inner . iter . size_hint () . 1) } }
    };
}

impl_276!()