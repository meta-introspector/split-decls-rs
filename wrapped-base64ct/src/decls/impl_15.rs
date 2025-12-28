macro_rules! deps {
    () => {
        Error!();
        BlockBuffer!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl BlockBuffer { # [doc = " Size of the buffer in bytes."] const SIZE : usize = 3 ; # [doc = " Fill the buffer by decoding up to 3 bytes of decoded Base64 input."] fn fill (& mut self , decoded_input : & [u8]) -> Result < () , Error > { debug_assert ! (self . is_empty ()) ; if decoded_input . len () > Self :: SIZE { return Err (InvalidLength) ; } self . position = 0 ; self . length = decoded_input . len () ; self . decoded [.. decoded_input . len ()] . copy_from_slice (decoded_input) ; Ok (()) } # [doc = " Take a specified number of bytes from the buffer."] # [doc = ""] # [doc = " Returns as many bytes as possible, or an empty slice if the buffer has"] # [doc = " already been read to completion."] fn take (& mut self , mut nbytes : usize) -> Result < & [u8] , Error > { debug_assert ! (self . position <= self . length) ; let start_pos = self . position ; let remaining_len = self . length . checked_sub (start_pos) . ok_or (InvalidLength) ? ; if nbytes > remaining_len { nbytes = remaining_len ; } self . position = self . position . checked_add (nbytes) . ok_or (InvalidLength) ? ; Ok (& self . decoded [start_pos ..] [.. nbytes]) } # [doc = " Have all of the bytes in this buffer been consumed?"] fn is_empty (& self) -> bool { self . position == self . length } }
    };
}

impl_15!();