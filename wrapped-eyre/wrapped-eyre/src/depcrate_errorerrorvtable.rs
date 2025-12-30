// Generated macro for ErrorVTable (struct)
macro_rules! Depcrate_errorErrorVTable {
() => {
// Module: crate::error
// Provides: {"ErrorVTable"}
// Dependencies: {}
struct ErrorVTable { object_drop : unsafe fn (OwnedPtr < ErrorImpl < () > >) , object_ref : unsafe fn (RefPtr < '_ , ErrorImpl < () > >) -> & (dyn StdError + Send + Sync + 'static) , object_mut : unsafe fn (MutPtr < '_ , ErrorImpl < () > >) -> & mut (dyn StdError + Send + Sync + 'static) , # [allow (clippy :: type_complexity)] object_boxed : unsafe fn (OwnedPtr < ErrorImpl < () > >) -> Box < dyn StdError + Send + Sync + 'static > , object_downcast : unsafe fn (RefPtr < '_ , ErrorImpl < () > > , TypeId) -> Option < NonNull < () > > , object_downcast_mut : unsafe fn (MutPtr < '_ , ErrorImpl < () > > , TypeId) -> Option < NonNull < () > > , object_drop_rest : unsafe fn (OwnedPtr < ErrorImpl < () > > , TypeId) , }
};
}
