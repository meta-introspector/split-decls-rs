// Generated macro for impl_290 (impl)
macro_rules! Depcrate_future_maybe_doneimpl_290 {
() => {
// Module: crate::future::maybe_done
// Provides: {"impl_290"}
// Dependencies: {}
impl < Fut : Future > MaybeDone < Fut > { # [doc = " Returns an [`Option`] containing a mutable reference to the output of the future."] # [doc = " The output of this method will be [`Some`] if and only if the inner"] # [doc = " future has been completed and [`take_output`](MaybeDone::take_output)"] # [doc = " has not yet been called."] # [inline] pub fn output_mut (self : Pin < & mut Self >) -> Option < & mut Fut :: Output > { unsafe { match self . get_unchecked_mut () { Self :: Done (res) => Some (res) , _ => None , } } } # [doc = " Attempt to take the output of a `MaybeDone` without driving it"] # [doc = " towards completion."] # [inline] pub fn take_output (self : Pin < & mut Self >) -> Option < Fut :: Output > { match & * self { Self :: Done (_) => { } Self :: Future (_) | Self :: Gone => return None , } unsafe { match mem :: replace (self . get_unchecked_mut () , Self :: Gone) { Self :: Done (output) => Some (output) , _ => unreachable ! () , } } } }
};
}
