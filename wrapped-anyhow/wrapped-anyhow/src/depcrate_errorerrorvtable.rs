// Generated macro for ErrorVTable (struct)
macro_rules! Depcrate_errorErrorVTable {
() => {
// Module: crate::error
// Provides: {"ErrorVTable"}
// Dependencies: {}
struct ErrorVTable { object_drop : unsafe fn (Own < ErrorImpl >) , object_ref : unsafe fn (Ref < ErrorImpl >) -> Ref < dyn StdError + Send + Sync + 'static > , # [cfg (all (any (feature = "std" , not (anyhow_no_core_error)) , anyhow_no_ptr_addr_of))] object_mut : unsafe fn (Mut < ErrorImpl >) -> & mut (dyn StdError + Send + Sync + 'static) , # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] object_boxed : unsafe fn (Own < ErrorImpl >) -> Box < dyn StdError + Send + Sync + 'static > , # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] object_reallocate_boxed : unsafe fn (Own < ErrorImpl >) -> Box < dyn StdError + Send + Sync + 'static > , object_downcast : unsafe fn (Ref < ErrorImpl > , TypeId) -> Option < Ref < () > > , # [cfg (anyhow_no_ptr_addr_of)] object_downcast_mut : unsafe fn (Mut < ErrorImpl > , TypeId) -> Option < Mut < () > > , object_drop_rest : unsafe fn (Own < ErrorImpl > , TypeId) , # [cfg (all (not (error_generic_member_access) , any (std_backtrace , feature = "backtrace")))] object_backtrace : unsafe fn (Ref < ErrorImpl >) -> Option < & Backtrace > , }
};
}
