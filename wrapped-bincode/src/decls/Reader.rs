macro_rules! deps {
    () => {
        DecodeError!();
    };
}

macro_rules! Reader {
    () => {
        deps!();
        # [doc = " A reader for owned data. See the module documentation for more information."] pub trait Reader { # [doc = " Fill the given `bytes` argument with values. Exactly the length of the given slice must be filled, or else an error must be returned."] fn read (& mut self , bytes : & mut [u8]) -> Result < () , DecodeError > ; # [doc = " If this reader wraps a buffer of any kind, this function lets callers access contents of"] # [doc = " the buffer without passing data through a buffer first."] # [inline] fn peek_read (& mut self , _ : usize) -> Option < & [u8] > { None } # [doc = " If an implementation of `peek_read` is provided, an implementation of this function"] # [doc = " must be provided so that subsequent reads or peek-reads do not return the same bytes"] # [inline] fn consume (& mut self , _ : usize) { } }
    };
}

Reader!();