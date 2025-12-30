// Generated macro for mask_for_bits (function)
macro_rules! Depcratemask_for_bits {
() => {
// Module: crate
// Provides: {"mask_for_bits"}
// Dependencies: {}
# [doc = " Computes the bitmask for the final word of the vector"] fn mask_for_bits < B : BitBlock > (bits : usize) -> B { (! B :: zero ()) >> ((B :: bits () - bits % B :: bits ()) % B :: bits ()) }
};
}
