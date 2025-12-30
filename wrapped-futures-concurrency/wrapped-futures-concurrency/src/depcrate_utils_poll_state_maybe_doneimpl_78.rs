// Generated macro for impl_78 (impl)
macro_rules! Depcrate_utils_poll_state_maybe_doneimpl_78 {
() => {
// Module: crate::utils::poll_state::maybe_done
// Provides: {"impl_78"}
// Dependencies: {}
impl < T , E , Fut > MaybeDone < Fut > where Fut : Future < Output = Result < T , E > > , { # [doc = " Attempt to take the `Ok(output)` of a `MaybeDone` without driving it towards completion."] # [doc = " If the future is done but is an `Err(_)`, this will return `None`."] # [inline] pub (crate) fn take_ok (self : Pin < & mut Self >) -> Option < T > { let this = unsafe { self . get_unchecked_mut () } ; match this { MaybeDone :: Done (Ok (_)) => { } MaybeDone :: Done (Err (_)) | MaybeDone :: Future (_) | MaybeDone :: Gone => return None , } match mem :: replace (this , MaybeDone :: Gone) { MaybeDone :: Done (Ok (output)) => Some (output) , _ => unreachable ! () , } } # [doc = " Attempt to take the `Err(output)` of a `MaybeDone` without driving it towards completion."] # [doc = " If the future is done but is an `Ok(_)`, this will return `None`."] # [inline] pub (crate) fn take_err (self : Pin < & mut Self >) -> Option < E > { let this = unsafe { self . get_unchecked_mut () } ; match this { MaybeDone :: Done (Err (_)) => { } MaybeDone :: Done (Ok (_)) | MaybeDone :: Future (_) | MaybeDone :: Gone => return None , } match mem :: replace (this , MaybeDone :: Gone) { MaybeDone :: Done (Err (output)) => Some (output) , _ => unreachable ! () , } } }
};
}
