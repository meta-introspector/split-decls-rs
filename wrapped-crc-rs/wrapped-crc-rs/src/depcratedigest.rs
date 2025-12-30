// Generated macro for Digest (struct)
macro_rules! DepcrateDigest {
() => {
// Module: crate
// Provides: {"Digest"}
// Dependencies: {}
# [derive (Clone)] pub struct Digest < 'a , W : Width , I : Implementation = DefaultImpl > { crc : & 'a Crc < W , I > , value : W , }
};
}
