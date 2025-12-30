// Generated macro for is_primitive (function)
macro_rules! Depcrate_world_generatoris_primitive {
() => {
// Module: crate::world_generator
// Provides: {"is_primitive"}
// Dependencies: {}
pub fn is_primitive (ty : & Type) -> bool { matches ! (ty , Type :: U8 | Type :: S8 | Type :: U16 | Type :: S16 | Type :: U32 | Type :: S32 | Type :: U64 | Type :: S64 | Type :: F32 | Type :: F64) }
};
}
