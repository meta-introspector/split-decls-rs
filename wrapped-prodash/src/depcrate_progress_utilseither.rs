// Generated macro for Either (enum)
macro_rules! Depcrate_progress_utilsEither {
() => {
// Module: crate::progress::utils
// Provides: {"Either"}
// Dependencies: {}
# [doc = " An implementation of [`NestedProgress`] showing either one or the other implementation."] # [doc = ""] # [doc = " Useful in conjunction with [`Discard`] and a working implementation, making it as a form of `Option<Progress>` which"] # [doc = " can be passed to methods requiring `impl Progress`."] # [doc = " See [`DoOrDiscard`] for an incarnation of this."] # [allow (missing_docs)] pub enum Either < L , R > { Left (L) , Right (R) , }
};
}
