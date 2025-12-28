macro_rules! deps {
    () => {
        Convert!();
        Iterator!();
        FallibleIterator!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < T , E , I > FallibleIterator for Convert < I > where I : iter :: Iterator < Item = Result < T , E > > , { type Item = T ; type Error = E ; # [inline] fn next (& mut self) -> Result < Option < T > , E > { match self . 0 . next () { Some (Ok (i)) => Ok (Some (i)) , Some (Err (e)) => Err (e) , None => Ok (None) , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } # [inline] fn try_fold < B , E2 , F > (& mut self , init : B , mut f : F) -> Result < B , E2 > where E2 : From < E > , F : FnMut (B , T) -> Result < B , E2 > , { self . 0 . try_fold (init , | acc , v | f (acc , v ?)) } }
    };
}

impl_68!();