// Generated macro for from_raw_const (function)
macro_rules! Depcrate_signaturefrom_raw_const {
() => {
// Module: crate::signature
// Provides: {"from_raw_const"}
// Dependencies: {}
# [doc = " Creates a new signature from the give raw pointer, tied to the lifetime"] # [doc = " of the given object."] # [doc = ""] # [doc = " This function is unsafe as there is no guarantee that `raw` is valid for"] # [doc = " `'a` nor if it's a valid pointer."] pub unsafe fn from_raw_const < 'b , T > (_lt : & 'b T , raw : * const raw :: git_signature) -> Signature < 'b > { Signature { raw : raw as * mut raw :: git_signature , _marker : marker :: PhantomData , owned : false , } }
};
}
