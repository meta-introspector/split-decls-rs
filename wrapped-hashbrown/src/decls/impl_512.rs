macro_rules! deps {
    () => {
        IterHashMut!();
    };
}

macro_rules! impl_512 {
    () => {
        deps!();
        impl < 'a , T > Iterator for IterHashMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < Self :: Item > { match self . inner . next () { Some (bucket) => Some (unsafe { bucket . as_mut () }) , None => None , } } fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , bucket | unsafe { f (acc , bucket . as_mut ()) }) } }
    };
}

impl_512!()