// Generated macro for hash_to_field (function)
macro_rules! Depcrate_hash2fieldhash_to_field {
() => {
// Module: crate::hash2field
// Provides: {"hash_to_field"}
// Dependencies: {}
# [doc = " Convert an arbitrary byte sequence into a field element."] # [doc = ""] # [doc = " <https://www.rfc-editor.org/rfc/rfc9380.html#name-hash_to_field-implementatio>"] # [doc = ""] # [doc = " For the `expand_message` call, `len_in_bytes = L * N`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns an error if the [`ExpandMsg`] implementation fails."] # [doc (hidden)] pub fn hash_to_field < const N : usize , E , K , T , L > (data : & [& [u8]] , domain : & [& [u8]] ,) -> Result < [T ; N] , E :: Error > where E : ExpandMsg < K > , T : Reduce < Array < u8 , L > > + Default , L : ArraySize + NonZero , { let len_in_bytes = const { assert ! (L :: USIZE . saturating_mul (N) <= u16 :: MAX as usize , "The product of `L` and `N` must not exceed `u16::MAX`.") ; NonZeroU16 :: new (L :: U16 * N as u16) . expect ("N is greater than 0") } ; let mut tmp = Array :: < u8 , L > :: default () ; let mut expander = E :: expand_message (data , domain , len_in_bytes) ? ; Ok (core :: array :: from_fn (| _ | { expander . fill_bytes (& mut tmp) . expect ("never exceeds `len_in_bytes`") ; T :: reduce (& tmp) })) }
};
}
