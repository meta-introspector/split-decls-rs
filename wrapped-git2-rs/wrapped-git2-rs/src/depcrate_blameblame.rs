// Generated macro for Blame (struct)
macro_rules! Depcrate_blameBlame {
() => {
// Module: crate::blame
// Provides: {"Blame"}
// Dependencies: {}
# [doc = " Opaque structure to hold blame results."] pub struct Blame < 'repo > { raw : * mut raw :: git_blame , _marker : marker :: PhantomData < & 'repo Repository > , }
};
}
