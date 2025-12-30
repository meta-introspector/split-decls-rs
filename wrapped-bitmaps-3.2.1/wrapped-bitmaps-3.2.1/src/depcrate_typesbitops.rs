// Generated macro for BitOps (trait)
macro_rules! Depcrate_typesBitOps {
() => {
// Module: crate::types
// Provides: {"BitOps"}
// Dependencies: {}
# [doc = " A trait that defines generalised operations on a `Bits::Store` type."] pub trait BitOps { fn get (bits : & Self , index : usize) -> bool ; fn set (bits : & mut Self , index : usize , value : bool) -> bool ; fn len (bits : & Self) -> usize ; fn first_index (bits : & Self) -> Option < usize > ; fn first_false_index (bits : & Self) -> Option < usize > ; fn last_index (bits : & Self) -> Option < usize > ; fn last_false_index (bits : & Self) -> Option < usize > ; fn next_index (bits : & Self , index : usize) -> Option < usize > ; fn next_false_index (bits : & Self , index : usize) -> Option < usize > ; fn prev_index (bits : & Self , index : usize) -> Option < usize > ; fn prev_false_index (bits : & Self , index : usize) -> Option < usize > ; fn bit_and (bits : & mut Self , other_bits : & Self) ; fn bit_or (bits : & mut Self , other_bits : & Self) ; fn bit_xor (bits : & mut Self , other_bits : & Self) ; fn invert (bits : & mut Self) ; fn make_mask (shift : usize) -> Self ; fn bit_size () -> usize ; # [cfg (feature = "std")] fn to_hex (bits : & Self) -> String ; }
};
}
