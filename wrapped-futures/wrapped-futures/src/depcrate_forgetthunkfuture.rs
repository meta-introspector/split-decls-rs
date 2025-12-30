// Generated macro for ThunkFuture (struct)
macro_rules! Depcrate_forgetThunkFuture {
() => {
// Module: crate::forget
// Provides: {"ThunkFuture"}
// Dependencies: {}
struct ThunkFuture < T , E > { inner : Box < Future < Item = T , Error = E > > , }
};
}
