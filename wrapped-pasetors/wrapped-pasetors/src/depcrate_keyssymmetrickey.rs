// Generated macro for SymmetricKey (struct)
macro_rules! Depcrate_keysSymmetricKey {
() => {
// Module: crate::keys
// Provides: {"SymmetricKey"}
// Dependencies: {}
# [derive (Clone)] # [doc = " A symmetric key used for `.local` tokens, given a version `V`."] pub struct SymmetricKey < V > { pub (crate) bytes : Vec < u8 > , pub (crate) phantom : PhantomData < V > , }
};
}
