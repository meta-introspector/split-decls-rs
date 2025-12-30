// Generated macro for Normalized (struct)
macro_rules! DepcrateNormalized {
() => {
// Module: crate
// Provides: {"Normalized"}
// Dependencies: {}
# [doc = " This struct wraps a `std::io::Chars` to normalize line endings."] # [doc = ""] # [doc = " Implements `Iterator<Item=char>` so can be used in place"] struct Normalized < I > { iter : I , prev_was_cr : bool , }
};
}
