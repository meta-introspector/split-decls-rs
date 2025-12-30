// Generated macro for _merge_errors (function)
macro_rules! Depcrate_error_merge_errors {
() => {
// Module: crate::error
// Provides: {"_merge_errors"}
// Dependencies: {}
pub (crate) fn _merge_errors < R1 , R2 > (r1 : Result < R1 , ErrorsVec > , r2 : Result < R2 , ErrorsVec > ,) -> Result < (R1 , R2) , ErrorsVec > { match (r1 , r2) { (Ok (r1) , Ok (r2)) => Ok ((r1 , r2)) , (Ok (_) , Err (e)) | (Err (e) , Ok (_)) => Err (e) , (Err (mut e1) , Err (mut e2)) => { e1 . append (& mut e2) ; Err (e1) } } }
};
}
