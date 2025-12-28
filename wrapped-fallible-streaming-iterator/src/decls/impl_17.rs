macro_rules! deps {
    () => {
        FuseState!();
        FallibleStreamingIterator!();
        Fuse!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < I > FallibleStreamingIterator for Fuse < I > where I : FallibleStreamingIterator , { type Item = I :: Item ; type Error = I :: Error ; # [inline] fn advance (& mut self) -> Result < () , I :: Error > { match self . state { FuseState :: Start => { match self . it . next () { Ok (Some (_)) => self . state = FuseState :: Middle , Ok (None) => self . state = FuseState :: End , Err (e) => { self . state = FuseState :: End ; return Err (e) ; } } ; } FuseState :: Middle => match self . it . next () { Ok (Some (_)) => { } Ok (None) => self . state = FuseState :: End , Err (e) => { self . state = FuseState :: End ; return Err (e) ; } } , FuseState :: End => { } } Ok (()) } # [inline] fn get (& self) -> Option < & I :: Item > { match self . state { FuseState :: Middle => self . it . get () , FuseState :: Start | FuseState :: End => None , } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } # [inline] fn next (& mut self) -> Result < Option < & I :: Item > , I :: Error > { match self . state { FuseState :: Start => match self . it . next () { Ok (Some (v)) => { self . state = FuseState :: Middle ; Ok (Some (v)) } Ok (None) => { self . state = FuseState :: End ; Ok (None) } Err (e) => { self . state = FuseState :: End ; Err (e) } } , FuseState :: Middle => match self . it . next () { Ok (Some (v)) => Ok (Some (v)) , Ok (None) => { self . state = FuseState :: End ; Ok (None) } Err (e) => { self . state = FuseState :: End ; Err (e) } } , FuseState :: End => Ok (None) , } } }
    };
}

impl_17!()