macro_rules! deps {
    () => {
        FallibleIterator!();
        Inspect!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < I , F > FallibleIterator for Inspect < I , F > where I : FallibleIterator , F : FnMut (& I :: Item) -> Result < () , I :: Error > , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , I :: Error > { match self . it . next () ? { Some (i) => { (self . f) (& i) ? ; Ok (Some (i)) } None => Ok (None) , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } # [inline] fn try_fold < B , E , G > (& mut self , init : B , mut f : G) -> Result < B , E > where E : From < I :: Error > , G : FnMut (B , I :: Item) -> Result < B , E > , { let inspect = & mut self . f ; self . it . try_fold (init , | acc , v | { inspect (& v) ? ; f (acc , v) }) } }
    };
}

impl_93!();