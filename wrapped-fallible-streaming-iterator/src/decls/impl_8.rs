macro_rules! deps {
    () => {
        DoubleEndedFallibleStreamingIterator!();
        Convert!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a , I , T , E > DoubleEndedFallibleStreamingIterator for Convert < 'a , I , T > where I : DoubleEndedIterator < Item = Result < & 'a T , E > > , { # [inline] fn advance_back (& mut self) -> Result < () , E > { self . item = match self . it . next_back () { Some (Ok (v)) => Some (v) , Some (Err (e)) => return Err (e) , None => None , } ; Ok (()) } }
    };
}

impl_8!()