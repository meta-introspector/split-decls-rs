macro_rules! deps {
    () => {
        SymmetricDifference!();
    };
}

macro_rules! impl_449 {
    () => {
        deps!();
        impl < 'a , T , S , A > Iterator for SymmetricDifference < 'a , T , S , A > where T : Eq + Hash , S : BuildHasher , A : Allocator , { type Item = & 'a T ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < & 'a T > { self . iter . next () } # [cfg_attr (feature = "inline-more" , inline)] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } # [cfg_attr (feature = "inline-more" , inline)] fn fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . iter . fold (init , f) } }
    };
}

impl_449!();