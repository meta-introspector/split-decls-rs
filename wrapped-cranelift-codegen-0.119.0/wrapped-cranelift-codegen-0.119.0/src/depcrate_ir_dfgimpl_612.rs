// Generated macro for impl_612 (impl)
macro_rules! Depcrate_ir_dfgimpl_612 {
() => {
// Module: crate::ir::dfg
// Provides: {"impl_612"}
// Dependencies: {}
impl ValueDataPacked { const Y_SHIFT : u8 = 0 ; const Y_BITS : u8 = 24 ; const X_SHIFT : u8 = Self :: Y_SHIFT + Self :: Y_BITS ; const X_BITS : u8 = 24 ; const TYPE_SHIFT : u8 = Self :: X_SHIFT + Self :: X_BITS ; const TYPE_BITS : u8 = 14 ; const TAG_SHIFT : u8 = Self :: TYPE_SHIFT + Self :: TYPE_BITS ; const TAG_BITS : u8 = 2 ; const TAG_INST : u64 = 0 ; const TAG_PARAM : u64 = 1 ; const TAG_ALIAS : u64 = 2 ; const TAG_UNION : u64 = 3 ; fn make (tag : u64 , ty : Type , x : u32 , y : u32) -> ValueDataPacked { debug_assert ! (tag < (1 << Self :: TAG_BITS)) ; debug_assert ! (ty . repr () < (1 << Self :: TYPE_BITS)) ; let x = encode_narrow_field (x , Self :: X_BITS) ; let y = encode_narrow_field (y , Self :: Y_BITS) ; ValueDataPacked ((tag << Self :: TAG_SHIFT) | ((ty . repr () as u64) << Self :: TYPE_SHIFT) | ((x as u64) << Self :: X_SHIFT) | ((y as u64) << Self :: Y_SHIFT) ,) } # [inline (always)] fn field (self , shift : u8 , bits : u8) -> u64 { (self . 0 >> shift) & ((1 << bits) - 1) } # [inline (always)] fn ty (self) -> Type { let ty = self . field (ValueDataPacked :: TYPE_SHIFT , ValueDataPacked :: TYPE_BITS) as u16 ; Type :: from_repr (ty) } # [inline (always)] fn set_type (& mut self , ty : Type) { self . 0 &= ! (((1 << Self :: TYPE_BITS) - 1) << Self :: TYPE_SHIFT) ; self . 0 |= (ty . repr () as u64) << Self :: TYPE_SHIFT ; } }
};
}
