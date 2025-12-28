macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for Values < 'a , K , V > { type Item = & 'a V ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < & 'a V > { match self . inner . next () { Some ((_ , v)) => Some (v) , None => None , } } # [cfg_attr (feature = "inline-more" , inline)] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [cfg_attr (feature = "inline-more" , inline)] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , (_ , v) | f (acc , v)) } }
    };
}

impl_316!()