macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_493 {
    () => {
        deps!();
        impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { match self . inner . next () { Some (bucket) => Some (unsafe { bucket . as_ref () }) , None => None , } } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , bucket | unsafe { f (acc , bucket . as_ref ()) }) } }
    };
}

impl_493!();