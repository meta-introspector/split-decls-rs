macro_rules! deps {
    () => {
        LineWrapper!();
        LineEnding!();
        Error!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl LineWrapper { # [doc = " Create a new linewrapper."] fn new (width : usize , ending : LineEnding) -> Result < Self , Error > { if width < MIN_LINE_WIDTH { return Err (InvalidLength) ; } Ok (Self { remaining : width , width , ending , }) } # [doc = " Wrap the number of blocks to encode near/at EOL."] fn wrap_blocks (& self , blocks : & mut usize) -> Result < () , Error > { if blocks . checked_mul (4) . ok_or (InvalidLength) ? >= self . remaining { * blocks = self . remaining / 4 ; } Ok (()) } # [doc = " Insert newlines into the output buffer as needed."] fn insert_newlines (& mut self , mut buffer : & mut [u8] , len : & mut usize) -> Result < () , Error > { let mut buffer_len = * len ; if buffer_len <= self . remaining { self . remaining = self . remaining . checked_sub (buffer_len) . ok_or (InvalidLength) ? ; return Ok (()) ; } buffer = & mut buffer [self . remaining ..] ; buffer_len = buffer_len . checked_sub (self . remaining) . ok_or (InvalidLength) ? ; debug_assert ! (buffer_len <= 4 , "buffer too long: {buffer_len}") ; let buffer_end = buffer_len . checked_add (self . ending . len ()) . ok_or (InvalidLength) ? ; if buffer_end >= buffer . len () { return Err (InvalidLength) ; } for i in (0 .. buffer_len) . rev () { buffer [i . checked_add (self . ending . len ()) . ok_or (InvalidLength) ?] = buffer [i] ; } buffer [.. self . ending . len ()] . copy_from_slice (self . ending . as_bytes ()) ; * len = (* len) . checked_add (self . ending . len ()) . ok_or (InvalidLength) ? ; self . remaining = self . width . checked_sub (buffer_len) . ok_or (InvalidLength) ? ; Ok (()) } }
    };
}

impl_30!();