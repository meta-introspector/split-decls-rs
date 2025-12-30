// Generated macro for Status (enum)
macro_rules! Depcrate_memStatus {
() => {
// Module: crate::mem
// Provides: {"Status"}
// Dependencies: {}
# [doc = " Possible status results of compressing some data or successfully"] # [doc = " decompressing a block of data."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum Status { # [doc = " Indicates success."] # [doc = ""] # [doc = " Means that more input may be needed but isn't available"] # [doc = " and/or there's more output to be written but the output buffer is full."] Ok , # [doc = " Indicates that forward progress is not possible due to input or output"] # [doc = " buffers being empty."] # [doc = ""] # [doc = " For compression it means the input buffer needs some more data or the"] # [doc = " output buffer needs to be freed up before trying again."] # [doc = ""] # [doc = " For decompression this means that more input is needed to continue or"] # [doc = " the output buffer isn't large enough to contain the result. The function"] # [doc = " can be called again after fixing both."] BufError , # [doc = " Indicates that all input has been consumed and all output bytes have"] # [doc = " been written. Decompression/compression should not be called again."] # [doc = ""] # [doc = " For decompression with zlib streams the adler-32 of the decompressed"] # [doc = " data has also been verified."] StreamEnd , }
};
}
