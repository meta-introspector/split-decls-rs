macro_rules! deps {
    () => {
        IterHash!();
    };
}

macro_rules! impl_506 {
    () => {
        deps!();
        impl < 'a , T > Iterator for IterHash < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { match self . inner . next () { Some (bucket) => Some (unsafe { bucket . as_ref () }) , None => None , } } fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , bucket | unsafe { f (acc , bucket . as_ref ()) }) } }
    };
}

impl_506!();