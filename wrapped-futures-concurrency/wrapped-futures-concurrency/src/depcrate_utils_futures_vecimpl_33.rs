// Generated macro for impl_33 (impl)
macro_rules! Depcrate_utils_futures_vecimpl_33 {
() => {
// Module: crate::utils::futures::vec
// Provides: {"impl_33"}
// Dependencies: {}
impl < T > FutureVec < T > { # [doc = " Create a new instance of `FutureVec`"] pub (crate) fn new (futures : Vec < T >) -> Self { let futures = MaybeUninit :: new (futures) ; let futures = unsafe { mem :: transmute_copy (& mem :: ManuallyDrop :: new (futures)) } ; Self { futures } } # [doc = " Create an iterator of pinned references."] pub (crate) fn iter (self : Pin < & mut Self >) -> impl Iterator < Item = Pin < & mut ManuallyDrop < T > > > { unsafe { self . get_unchecked_mut () } . futures . iter_mut () . map (| t | unsafe { Pin :: new_unchecked (t) }) } # [doc = " Drop a future at the given index."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The future is held in a `ManuallyDrop`, so no double-dropping, etc"] pub (crate) unsafe fn drop (mut self : Pin < & mut Self > , idx : usize) { unsafe { let futures = self . as_mut () . get_unchecked_mut () . futures . as_mut_slice () ; ManuallyDrop :: drop (& mut futures [idx]) ; } ; } }
};
}
