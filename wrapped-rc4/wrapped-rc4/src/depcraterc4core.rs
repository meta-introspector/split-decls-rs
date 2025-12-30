// Generated macro for Rc4Core (struct)
macro_rules! DepcrateRc4Core {
() => {
// Module: crate
// Provides: {"Rc4Core"}
// Dependencies: {}
# [doc = " Core state of the RC4 stream cipher initialized only with key."] pub struct Rc4Core < KeySize > { state : Rc4State , key_size : PhantomData < KeySize > , }
};
}
