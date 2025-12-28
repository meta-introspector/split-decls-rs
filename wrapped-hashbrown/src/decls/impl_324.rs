macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_324 {
    () => {
        deps!();
        impl < K , V , A : Allocator > Iterator for Drain < '_ , K , V , A > { type Item = (K , V) ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < (K , V) > { self . inner . next () } # [cfg_attr (feature = "inline-more" , inline)] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [cfg_attr (feature = "inline-more" , inline)] fn fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , f) } }
    };
}

impl_324!()