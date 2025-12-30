// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> std :: io :: Result < () > { let provider = DataProtectionProvider :: CreateOverloadExplicit (h ! ("LOCAL=user")) ? ; let unprotected = CryptographicBuffer :: ConvertStringToBinary (h ! ("Hello world") , BinaryStringEncoding :: Utf8) ? ; let protected = provider . ProtectAsync (& unprotected) ? . join () ? ; let protected_bytes = unsafe { as_mut_bytes (& protected) ? } ; std :: fs :: write ("secret.bin" , protected_bytes) ? ; let protected_bytes = std :: fs :: read ("secret.bin") ? ; let protected = CryptographicBuffer :: CreateFromByteArray (& protected_bytes) ? ; let unprotected = provider . UnprotectAsync (& protected) ? . join () ? ; let message = CryptographicBuffer :: ConvertBinaryToString (BinaryStringEncoding :: Utf8 , & unprotected) ? ; println ! ("{message:?}") ; Ok (()) }
};
}
