// Generated macro for int_type_for (function)
macro_rules! Depcrateint_type_for {
() => {
// Module: crate
// Provides: {"int_type_for"}
// Dependencies: {}
fn int_type_for (width : u8) -> & 'static str { let width : u32 = width . try_into () . unwrap () ; macro_rules ! int_tys { ($ ($ ty : ident) ,*) => { $ (if width <= $ ty :: BITS { return stringify ! ($ ty) ; }) * } ; } int_tys ! (u8 , u16 , u32 , u64 , u128) ; panic ! ("No unsigned int type for width: {width}") ; }
};
}
