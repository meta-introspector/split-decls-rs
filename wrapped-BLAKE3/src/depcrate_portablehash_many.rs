// Generated macro for hash_many (function)
macro_rules! Depcrate_portablehash_many {
() => {
// Module: crate::portable
// Provides: {"hash_many"}
// Dependencies: {}
pub fn hash_many < const N : usize > (inputs : & [& [u8 ; N]] , key : & CVWords , mut counter : u64 , increment_counter : IncrementCounter , flags : u8 , flags_start : u8 , flags_end : u8 , out : & mut [u8] ,) { debug_assert ! (out . len () >= inputs . len () * OUT_LEN , "out too short") ; for (& input , output) in inputs . iter () . zip (out . chunks_exact_mut (OUT_LEN)) { hash1 (input , key , counter , flags , flags_start , flags_end , array_mut_ref ! (output , 0 , OUT_LEN) ,) ; if increment_counter . yes () { counter += 1 ; } } }
};
}
