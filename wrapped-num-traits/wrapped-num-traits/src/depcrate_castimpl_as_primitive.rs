// Generated macro for impl_as_primitive (macro)
macro_rules! Depcrate_castimpl_as_primitive {
() => {
// Module: crate::cast
// Provides: {"impl_as_primitive"}
// Dependencies: {}
macro_rules ! impl_as_primitive { (@ $ T : ty => impl $ U : ty) => { impl AsPrimitive <$ U > for $ T { # [inline] fn as_ (self) -> $ U { self as $ U } } } ; (@ $ T : ty => { $ ($ U : ty) ,* }) => { $ (impl_as_primitive ! (@ $ T => impl $ U) ;) * } ; ($ T : ty => { $ ($ U : ty) ,* }) => { impl_as_primitive ! (@ $ T => { $ ($ U) ,* }) ; impl_as_primitive ! (@ $ T => { u8 , u16 , u32 , u64 , u128 , usize }) ; impl_as_primitive ! (@ $ T => { i8 , i16 , i32 , i64 , i128 , isize }) ; } ; }
};
}
