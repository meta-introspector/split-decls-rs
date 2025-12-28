macro_rules! deps {
    () => {
        FieldsWith!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < 'a , F : FnMut (char) -> bool > Iterator for FieldsWith < 'a , F > { type Item = & 'a [u8] ; # [inline] fn next (& mut self) -> Option < & 'a [u8] > { let (start , mut end) ; loop { match self . chars . next () { None => return None , Some ((s , e , ch)) => { if ! (self . f) (ch) { start = s ; end = e ; break ; } } } } for (_ , e , ch) in self . chars . by_ref () { if (self . f) (ch) { break ; } end = e ; } Some (& self . bytes [start .. end]) } }
    };
}

impl_87!()