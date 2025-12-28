macro_rules! deps {
    () => {
        BorrowReader!();
        DecodeError!();
        SliceReader!();
    };
}

macro_rules! impl_398 {
    () => {
        deps!();
        impl < 'storage > BorrowReader < 'storage > for SliceReader < 'storage > { # [inline (always)] fn take_bytes (& mut self , length : usize) -> Result < & 'storage [u8] , DecodeError > { if length > self . slice . len () { return Err (DecodeError :: UnexpectedEnd { additional : length - self . slice . len () , }) ; } let (read_slice , remaining) = self . slice . split_at (length) ; self . slice = remaining ; Ok (read_slice) } }
    };
}

impl_398!()