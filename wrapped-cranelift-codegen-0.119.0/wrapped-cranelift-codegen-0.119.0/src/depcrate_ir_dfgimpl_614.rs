// Generated macro for impl_614 (impl)
macro_rules! Depcrate_ir_dfgimpl_614 {
() => {
// Module: crate::ir::dfg
// Provides: {"impl_614"}
// Dependencies: {}
impl From < ValueDataPacked > for ValueData { fn from (data : ValueDataPacked) -> Self { let tag = data . field (ValueDataPacked :: TAG_SHIFT , ValueDataPacked :: TAG_BITS) ; let ty = u16 :: try_from (data . field (ValueDataPacked :: TYPE_SHIFT , ValueDataPacked :: TYPE_BITS)) . expect ("Mask should ensure result fits in a u16") ; let x = u32 :: try_from (data . field (ValueDataPacked :: X_SHIFT , ValueDataPacked :: X_BITS)) . expect ("Mask should ensure result fits in a u32") ; let y = u32 :: try_from (data . field (ValueDataPacked :: Y_SHIFT , ValueDataPacked :: Y_BITS)) . expect ("Mask should ensure result fits in a u32") ; let ty = Type :: from_repr (ty) ; match tag { ValueDataPacked :: TAG_INST => ValueData :: Inst { ty , num : u16 :: try_from (x) . expect ("Inst result num should fit in u16") , inst : Inst :: from_bits (decode_narrow_field (y , ValueDataPacked :: Y_BITS)) , } , ValueDataPacked :: TAG_PARAM => ValueData :: Param { ty , num : u16 :: try_from (x) . expect ("Blockparam index should fit in u16") , block : Block :: from_bits (decode_narrow_field (y , ValueDataPacked :: Y_BITS)) , } , ValueDataPacked :: TAG_ALIAS => ValueData :: Alias { ty , original : Value :: from_bits (decode_narrow_field (y , ValueDataPacked :: Y_BITS)) , } , ValueDataPacked :: TAG_UNION => ValueData :: Union { ty , x : Value :: from_bits (decode_narrow_field (x , ValueDataPacked :: X_BITS)) , y : Value :: from_bits (decode_narrow_field (y , ValueDataPacked :: Y_BITS)) , } , _ => panic ! ("Invalid tag {} in ValueDataPacked 0x{:x}" , tag , data . 0) , } } }
};
}
