macro_rules! deps {
    () => {
        FallibleIterator!();
        Scan!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < B , I , St , F > FallibleIterator for Scan < I , St , F > where I : FallibleIterator , F : FnMut (& mut St , I :: Item) -> Result < Option < B > , I :: Error > , { type Item = B ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < B > , I :: Error > { match self . it . next () ? { Some (v) => (self . f) (& mut self . state , v) , None => Ok (None) , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let hint = self . it . size_hint () ; (0 , hint . 1) } }
    };
}

impl_110!();