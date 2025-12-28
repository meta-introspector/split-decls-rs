macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        impl < 'a , K > Iterator for Iter < 'a , K > { type Item = & 'a K ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < & 'a K > { self . iter . next () } # [cfg_attr (feature = "inline-more" , inline)] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } # [cfg_attr (feature = "inline-more" , inline)] fn fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . iter . fold (init , f) } }
    };
}

impl_425!();