// Generated macro for SharedData (struct)
macro_rules! Depcrate_serializeSharedData {
() => {
// Module: crate::serialize
// Provides: {"SharedData"}
// Dependencies: {}
# [doc = " Shared (SQLITE_SERIALIZE_NOCOPY) serialized database"] pub struct SharedData < 'conn > { phantom : PhantomData < & 'conn Connection > , ptr : NonNull < u8 > , sz : usize , }
};
}
