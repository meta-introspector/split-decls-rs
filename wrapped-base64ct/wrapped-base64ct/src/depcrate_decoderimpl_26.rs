// Generated macro for impl_26 (impl)
macro_rules! Depcrate_decoderimpl_26 {
() => {
// Module: crate::decoder
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'i > Line < 'i > { # [doc = " Create a new line which wraps the given input data."] fn new (bytes : & 'i [u8]) -> Self { Self { remaining : bytes } } # [doc = " Take up to `nbytes` from this line buffer."] fn take (& mut self , nbytes : usize) -> & 'i [u8] { let (bytes , rest) = if nbytes < self . remaining . len () { self . remaining . split_at (nbytes) } else { (self . remaining , [] . as_ref ()) } ; self . remaining = rest ; bytes } # [doc = " Slice off a tail of a given length."] fn slice_tail (& self , nbytes : usize) -> Result < & 'i [u8] , Error > { let offset = self . len () . checked_sub (nbytes) . ok_or (InvalidLength) ? ; self . remaining . get (offset ..) . ok_or (InvalidLength) } # [doc = " Get the number of bytes remaining in this line."] fn len (& self) -> usize { self . remaining . len () } # [doc = " Is the buffer for this line empty?"] fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Trim the newline off the end of this line."] fn trim_end (& self) -> Self { Line :: new (match self . remaining { [line @ .. , CHAR_CR , CHAR_LF] => line , [line @ .. , CHAR_CR] => line , [line @ .. , CHAR_LF] => line , line => line , }) } }
};
}
