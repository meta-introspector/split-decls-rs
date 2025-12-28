macro_rules! deps {
    () => {
        Fuse!();
        FallibleIterator!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < I > FallibleIterator for Fuse < I > where I : FallibleIterator , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn next (& mut self) -> Result < Option < I :: Item > , I :: Error > { if self . done { return Ok (None) ; } match self . it . next () ? { Some (i) => Ok (Some (i)) , None => { self . done = true ; Ok (None) } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { if self . done { (0 , Some (0)) } else { self . it . size_hint () } } # [inline] fn count (self) -> Result < usize , I :: Error > { if self . done { Ok (0) } else { self . it . count () } } # [inline] fn last (self) -> Result < Option < I :: Item > , I :: Error > { if self . done { Ok (None) } else { self . it . last () } } # [inline] fn nth (& mut self , n : usize) -> Result < Option < I :: Item > , I :: Error > { if self . done { Ok (None) } else { let v = self . it . nth (n) ? ; if v . is_none () { self . done = true ; } Ok (v) } } # [inline] fn try_fold < B , E , F > (& mut self , init : B , f : F) -> Result < B , E > where E : From < I :: Error > , F : FnMut (B , I :: Item) -> Result < B , E > , { if self . done { Ok (init) } else { self . it . try_fold (init , f) } } }
    };
}

impl_50!()