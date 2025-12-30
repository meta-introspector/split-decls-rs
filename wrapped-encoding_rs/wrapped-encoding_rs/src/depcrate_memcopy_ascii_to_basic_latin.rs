// Generated macro for copy_ascii_to_basic_latin (function)
macro_rules! Depcrate_memcopy_ascii_to_basic_latin {
() => {
// Module: crate::mem
// Provides: {"copy_ascii_to_basic_latin"}
// Dependencies: {}
# [doc = " Copies ASCII from source to destination zero-extending it to UTF-16 up to"] # [doc = " the first non-ASCII byte (or the end of the input if it is ASCII in its"] # [doc = " entirety)."] # [doc = ""] # [doc = " The length of the destination buffer must be at least the length of the"] # [doc = " source buffer."] # [doc = ""] # [doc = " Returns the number of `u16`s written."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the destination buffer is shorter than stated above."] pub fn copy_ascii_to_basic_latin (src : & [u8] , dst : & mut [u16]) -> usize { assert ! (dst . len () >= src . len () , "Destination must not be shorter than the source.") ; if let Some ((_ , consumed)) = unsafe { ascii_to_basic_latin (src . as_ptr () , dst . as_mut_ptr () , src . len ()) } { consumed } else { src . len () } }
};
}
