macro_rules! deps {
    () => {
        TakeWhileInclusive!();
    };
}

macro_rules! impl_493 {
    () => {
        deps!();
        impl < I , F > Iterator for TakeWhileInclusive < I , F > where I : Iterator , F : FnMut (& I :: Item) -> bool , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { if self . done { None } else { self . iter . next () . map (| item | { if ! (self . predicate) (& item) { self . done = true ; } item }) } } fn size_hint (& self) -> (usize , Option < usize >) { if self . done { (0 , Some (0)) } else { (0 , self . iter . size_hint () . 1) } } fn fold < B , Fold > (mut self , init : B , mut f : Fold) -> B where Fold : FnMut (B , Self :: Item) -> B , { if self . done { init } else { let predicate = & mut self . predicate ; self . iter . try_fold (init , | mut acc , item | { let is_ok = predicate (& item) ; acc = f (acc , item) ; if is_ok { Ok (acc) } else { Err (acc) } }) . unwrap_or_else (| err | err) } } }
    };
}

impl_493!();