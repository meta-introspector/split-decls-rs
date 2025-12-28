macro_rules! deps {
    () => {
        Difference!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        impl < 'a , T , S , A > Iterator for Difference < 'a , T , S , A > where T : Eq + Hash , S : BuildHasher , A : Allocator , { type Item = & 'a T ; # [cfg_attr (feature = "inline-more" , inline)] fn next (& mut self) -> Option < & 'a T > { loop { let elt = self . iter . next () ? ; if ! self . other . contains (elt) { return Some (elt) ; } } } # [cfg_attr (feature = "inline-more" , inline)] fn size_hint (& self) -> (usize , Option < usize >) { let (lower , upper) = self . iter . size_hint () ; (lower . saturating_sub (self . other . len ()) , upper) } # [cfg_attr (feature = "inline-more" , inline)] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . iter . fold (init , | acc , elt | { if self . other . contains (elt) { acc } else { f (acc , elt) } }) } }
    };
}

impl_445!()