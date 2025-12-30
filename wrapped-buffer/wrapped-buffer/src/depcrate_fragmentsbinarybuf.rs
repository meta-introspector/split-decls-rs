// Generated macro for BinaryBuf (struct)
macro_rules! Depcrate_fragmentsBinaryBuf {
() => {
// Module: crate::fragments
// Provides: {"BinaryBuf"}
// Dependencies: {}
# [doc = "\nBuffer binary fragments into a single contiguous slice.\n\nIn no-std environments, this buffer only supports a single\nborrowed binary fragment. Other methods will fail.\n"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct BinaryBuf < 'sval > { buf : FragmentBuf < 'sval , [u8] > , }
};
}
