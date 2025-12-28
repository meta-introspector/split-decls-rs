macro_rules! deps {
    () => {
        LazyBuffer!();
        SizeHint!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl < I > LazyBuffer < I > where I : Iterator , { pub fn new (it : I) -> Self { Self { it : it . fuse () , buffer : Vec :: new () , } } pub fn len (& self) -> usize { self . buffer . len () } pub fn size_hint (& self) -> SizeHint { size_hint :: add_scalar (self . it . size_hint () , self . len ()) } pub fn count (self) -> usize { self . len () + self . it . count () } pub fn get_next (& mut self) -> bool { if let Some (x) = self . it . next () { self . buffer . push (x) ; true } else { false } } pub fn prefill (& mut self , len : usize) { let buffer_len = self . buffer . len () ; if len > buffer_len { let delta = len - buffer_len ; self . buffer . extend (self . it . by_ref () . take (delta)) ; } } }
    };
}

impl_340!()