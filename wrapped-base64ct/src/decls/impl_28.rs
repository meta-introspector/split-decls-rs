macro_rules! deps {
    () => {
        Error!();
        BlockBuffer!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl BlockBuffer { # [doc = " Size of the buffer in bytes: 3-bytes of unencoded input which"] # [doc = " Base64 encode to 4-bytes of output."] const SIZE : usize = 3 ; # [doc = " Fill the remaining space in the buffer with the input data."] fn fill (& mut self , input : & mut & [u8]) -> Result < () , Error > { let remaining = Self :: SIZE . checked_sub (self . position) . ok_or (InvalidLength) ? ; let len = cmp :: min (input . len () , remaining) ; self . bytes [self . position ..] [.. len] . copy_from_slice (& input [.. len]) ; self . position = self . position . checked_add (len) . ok_or (InvalidLength) ? ; * input = & input [len ..] ; Ok (()) } # [doc = " Take the output buffer, resetting the position to 0."] fn take (& mut self) -> [u8 ; Self :: SIZE] { debug_assert ! (self . is_full ()) ; let result = self . bytes ; * self = Default :: default () ; result } # [doc = " Is the buffer empty?"] fn is_empty (& self) -> bool { self . position == 0 } # [doc = " Is the buffer full?"] fn is_full (& self) -> bool { self . position == Self :: SIZE } }
    };
}

impl_28!();