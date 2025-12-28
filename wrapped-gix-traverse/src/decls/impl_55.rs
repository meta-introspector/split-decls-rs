macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl State { # [doc = " Pop one empty buffer from the free-list."] pub fn pop_buf (& mut self) -> Vec < u8 > { match self . freelist . pop () { None => Vec :: new () , Some (mut buf) => { buf . clear () ; buf } } } # [doc = " Make `buf` available for re-use with [`Self::pop_buf()`]."] pub fn push_buf (& mut self , buf : Vec < u8 >) { self . freelist . push (buf) ; } }
    };
}

impl_55!()