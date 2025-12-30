// Generated macro for blocks_for_bits (function)
macro_rules! Depcrateblocks_for_bits {
() => {
// Module: crate
// Provides: {"blocks_for_bits"}
// Dependencies: {}
# [doc = " Computes how many blocks are needed to store that many bits"] fn blocks_for_bits < B : BitBlock > (bits : usize) -> usize { if bits % B :: bits () == 0 { bits / B :: bits () } else { bits / B :: bits () + 1 } }
};
}
