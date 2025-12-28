macro_rules! deps {
    () => {
        SizeHint!();
        Powerset!();
    };
}

macro_rules! impl_441 {
    () => {
        deps!();
        impl < I > Iterator for Powerset < I > where I : Iterator , I :: Item : Clone , { type Item = Vec < I :: Item > ; fn next (& mut self) -> Option < Self :: Item > { if let Some (elt) = self . combs . next () { Some (elt) } else if self . increment_k () { self . combs . next () } else { None } } fn nth (& mut self , mut n : usize) -> Option < Self :: Item > { loop { match self . combs . try_nth (n) { Ok (item) => return Some (item) , Err (steps) => { if ! self . increment_k () { return None ; } n -= steps ; } } } } fn size_hint (& self) -> SizeHint { let k = self . combs . k () ; let (n_min , n_max) = self . combs . src () . size_hint () ; let low = remaining_for (n_min , k) . unwrap_or (usize :: MAX) ; let upp = n_max . and_then (| n | remaining_for (n , k)) ; size_hint :: add (self . combs . size_hint () , (low , upp)) } fn count (self) -> usize { let k = self . combs . k () ; let (n , combs_count) = self . combs . n_and_count () ; combs_count + remaining_for (n , k) . unwrap () } fn fold < B , F > (self , mut init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { let mut it = self . combs ; if it . k () == 0 { init = it . by_ref () . fold (init , & mut f) ; it . reset (1) ; } init = it . by_ref () . fold (init , & mut f) ; for k in it . k () + 1 ..= it . n () { it . reset (k) ; init = it . by_ref () . fold (init , & mut f) ; } init } }
    };
}

impl_441!()