macro_rules! deps {
    () => {
        IntoFallible!();
        FallibleIterator!();
        Iterator!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < T , I > FallibleIterator for IntoFallible < I > where I : iter :: Iterator < Item = T > , { type Item = T ; type Error = Infallible ; # [inline] fn next (& mut self) -> Result < Option < T > , Self :: Error > { Ok (self . 0 . next ()) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } # [inline] fn try_fold < B , E2 , F > (& mut self , init : B , f : F) -> Result < B , E2 > where E2 : From < Infallible > , F : FnMut (B , T) -> Result < B , E2 > , { self . 0 . try_fold (init , f) } }
    };
}

impl_30!()