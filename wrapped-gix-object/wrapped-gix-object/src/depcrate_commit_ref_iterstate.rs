// Generated macro for State (enum)
macro_rules! Depcrate_commit_ref_iterState {
() => {
// Module: crate::commit::ref_iter
// Provides: {"State"}
// Dependencies: {}
# [derive (Default , Copy , Clone)] pub (crate) enum State { # [default] Tree , Parents , Signature { of : SignatureKind , } , Encoding , ExtraHeaders , Message , }
};
}
