macro_rules! deps {
    () => {
        Zip!();
        FallibleIterator!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < T , U > FallibleIterator for Zip < T , U > where T : FallibleIterator , U : FallibleIterator < Error = T :: Error > , { type Item = (T :: Item , U :: Item) ; type Error = T :: Error ; # [inline] fn next (& mut self) -> Result < Option < (T :: Item , U :: Item) > , T :: Error > { match (self . 0 . next () ? , self . 1 . next () ?) { (Some (a) , Some (b)) => Ok (Some ((a , b))) , _ => Ok (None) , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let a = self . 0 . size_hint () ; let b = self . 1 . size_hint () ; let low = cmp :: min (a . 0 , b . 0) ; let high = match (a . 1 , b . 1) { (Some (a) , Some (b)) => Some (cmp :: min (a , b)) , (Some (a) , None) => Some (a) , (None , Some (b)) => Some (b) , (None , None) => None , } ; (low , high) } }
    };
}

impl_124!()