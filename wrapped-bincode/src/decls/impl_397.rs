macro_rules! deps {
    () => {
        SliceReader!();
        Reader!();
        DecodeError!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        impl < 'storage > Reader for SliceReader < 'storage > { # [inline (always)] fn read (& mut self , bytes : & mut [u8]) -> Result < () , DecodeError > { if bytes . len () > self . slice . len () { return Err (DecodeError :: UnexpectedEnd { additional : bytes . len () - self . slice . len () , }) ; } let (read_slice , remaining) = self . slice . split_at (bytes . len ()) ; bytes . copy_from_slice (read_slice) ; self . slice = remaining ; Ok (()) } # [inline] fn peek_read (& mut self , n : usize) -> Option < & 'storage [u8] > { self . slice . get (.. n) } # [inline] fn consume (& mut self , n : usize) { self . slice = self . slice . get (n ..) . unwrap_or_default () ; } }
    };
}

impl_397!();