// Generated macro for impl_96 (impl)
macro_rules! Depcrate_once_cellimpl_96 {
() => {
// Module: crate::once_cell
// Provides: {"impl_96"}
// Dependencies: {}
impl < T > From < T > for OnceCell < T > { # [doc = " Create a new, initialized `OnceCell` from an existing value."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use async_lock::OnceCell;"] # [doc = ""] # [doc = " let cell = OnceCell::from(42);"] # [doc = " assert_eq!(cell.get(), Some(&42));"] # [doc = " ```"] fn from (value : T) -> Self { Self { active_initializers : Event :: new () , passive_waiters : Event :: new () , state : AtomicUsize :: new (State :: Initialized . into ()) , value : UnsafeCell :: new (MaybeUninit :: new (value)) , } } }
};
}
