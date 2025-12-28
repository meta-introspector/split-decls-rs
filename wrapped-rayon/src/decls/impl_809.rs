macro_rules! deps {
    () => {
        Empty!();
        RepeatNProducer!();
    };
}

macro_rules! impl_809 {
    () => {
        deps!();
        impl < T : Clone > Iterator for RepeatNProducer < T > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { if let Self :: Repeats (element , count) = self { if let Some (rem) = NonZeroUsize :: new (count . get () - 1) { * count = rem ; Some (element . clone ()) } else { match mem :: replace (self , Self :: Empty) { Self :: Repeats (element , _) => Some (element) , Self :: Empty => unreachable ! () , } } } else { None } } # [inline] fn nth (& mut self , n : usize) -> Option < T > { if let Self :: Repeats (_ , count) = self { if let Some (rem) = NonZeroUsize :: new (count . get () . saturating_sub (n)) { * count = rem ; return self . next () ; } * self = Self :: Empty ; } None } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
    };
}

impl_809!()