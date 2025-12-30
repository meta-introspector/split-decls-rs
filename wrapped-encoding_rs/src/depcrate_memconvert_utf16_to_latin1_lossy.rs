// Generated macro for convert_utf16_to_latin1_lossy (function)
macro_rules! Depcrate_memconvert_utf16_to_latin1_lossy {
() => {
// Module: crate::mem
// Provides: {"convert_utf16_to_latin1_lossy"}
// Dependencies: {}
# [doc = " If the input is valid UTF-16 representing only Unicode code points from"] # [doc = " U+0000 to U+00FF, inclusive, converts the input into output that"] # [doc = " represents the value of each code point as the unsigned byte value of"] # [doc = " each output byte."] # [doc = ""] # [doc = " If the input does not fulfill the condition stated above, does something"] # [doc = " that is memory-safe without any promises about any properties of the"] # [doc = " output and will probably assert in debug builds in future versions."] # [doc = " In particular, callers shouldn't assume the output to be the same across"] # [doc = " crate versions or CPU architectures and should not assume that non-ASCII"] # [doc = " input can't map to ASCII output."] # [doc = ""] # [doc = " The length of the destination buffer must be at least the length of the"] # [doc = " source buffer."] # [doc = ""] # [doc = " The number of bytes written equals the length of the source buffer."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the destination buffer is shorter than stated above."] # [doc = ""] # [doc = " (Probably in future versions if debug assertions are enabled (and not"] # [doc = " fuzzing) and the input is not in the range U+0000 to U+00FF, inclusive.)"] pub fn convert_utf16_to_latin1_lossy (src : & [u16] , dst : & mut [u8]) { assert ! (dst . len () >= src . len () , "Destination must not be shorter than the source.") ; unsafe { pack_latin1 (src . as_ptr () , dst . as_mut_ptr () , src . len ()) ; } }
};
}
