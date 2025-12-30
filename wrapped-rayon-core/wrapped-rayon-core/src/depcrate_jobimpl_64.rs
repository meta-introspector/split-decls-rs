// Generated macro for impl_64 (impl)
macro_rules! Depcrate_jobimpl_64 {
() => {
// Module: crate::job
// Provides: {"impl_64"}
// Dependencies: {}
impl JobRef { # [doc = " Unsafe: caller asserts that `data` will remain valid until the"] # [doc = " job is executed."] pub (super) unsafe fn new < T > (data : * const T) -> JobRef where T : Job , { JobRef { pointer : data as * const () , execute_fn : < T as Job > :: execute , } } # [doc = " Returns an opaque handle that can be saved and compared,"] # [doc = " without making `JobRef` itself `Copy + Eq`."] # [inline] pub (super) fn id (& self) -> JobId { (self . pointer , self . execute_fn) } # [inline] pub (super) unsafe fn execute (self) { unsafe { (self . execute_fn) (self . pointer) } } }
};
}
