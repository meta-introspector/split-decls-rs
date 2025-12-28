macro_rules! deps {
    () => {
        FallibleStreamingIterator!();
        Convert!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'a , I , T , E > FallibleStreamingIterator for Convert < 'a , I , T > where I : Iterator < Item = Result < & 'a T , E > > , { type Item = T ; type Error = E ; # [inline] fn advance (& mut self) -> Result < () , E > { self . item = match self . it . next () { Some (Ok (v)) => Some (v) , Some (Err (e)) => return Err (e) , None => None , } ; Ok (()) } # [inline] fn get (& self) -> Option < & T > { self . item } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_7!();