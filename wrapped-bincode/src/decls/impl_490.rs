macro_rules! deps {
    () => {
        SliceWriter!();
        EncodeError!();
        Writer!();
    };
}

macro_rules! impl_490 {
    () => {
        deps!();
        impl Writer for SliceWriter < '_ > { # [inline (always)] fn write (& mut self , bytes : & [u8]) -> Result < () , EncodeError > { if bytes . len () > self . slice . len () { return Err (EncodeError :: UnexpectedEnd) ; } let (a , b) = core :: mem :: take (& mut self . slice) . split_at_mut (bytes . len ()) ; a . copy_from_slice (bytes) ; self . slice = b ; Ok (()) } }
    };
}

impl_490!()