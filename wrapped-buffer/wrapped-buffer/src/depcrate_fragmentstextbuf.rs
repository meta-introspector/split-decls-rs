// Generated macro for TextBuf (struct)
macro_rules! Depcrate_fragmentsTextBuf {
() => {
// Module: crate::fragments
// Provides: {"TextBuf"}
// Dependencies: {}
# [doc = "\nBuffer text fragments into a single contiguous string.\n\nIn no-std environments, this buffer only supports a single\nborrowed text fragment. Other methods will fail.\n"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct TextBuf < 'sval > { buf : FragmentBuf < 'sval , str > , }
};
}
