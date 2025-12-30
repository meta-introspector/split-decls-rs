// Generated macro for AllocOrInitError (enum)
macro_rules! DepcrateAllocOrInitError {
() => {
// Module: crate
// Provides: {"AllocOrInitError"}
// Dependencies: {}
# [doc = " An error returned from [`Bump::try_alloc_try_with`]."] # [derive (Clone , PartialEq , Eq , Debug)] pub enum AllocOrInitError < E > { # [doc = " Indicates that the initial allocation failed."] Alloc (AllocErr) , # [doc = " Indicates that the initializer failed with the contained error after"] # [doc = " allocation."] # [doc = ""] # [doc = " It is possible but not guaranteed that the allocated memory has been"] # [doc = " released back to the allocator at this point."] Init (E) , }
};
}
