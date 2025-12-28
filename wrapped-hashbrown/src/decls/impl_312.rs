macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for Keys < 'a , K , V > { type Item = & 'a K ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < & 'a K > { match self . inner . next () { Some ((k , _)) => Some (k) , None => None , } } # [cfg_attr (feature = "inline-more" , inline)] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [cfg_attr (feature = "inline-more" , inline)] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , (k , _) | f (acc , k)) } }
    };
}

impl_312!();