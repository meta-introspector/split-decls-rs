// Generated macro for Scan (struct)
macro_rules! DepcrateScan {
() => {
// Module: crate
// Provides: {"Scan"}
// Dependencies: {}
# [doc = " An iterator which applies a stateful closure."] # [derive (Clone , Debug)] pub struct Scan < I , St , F > { it : I , f : F , state : St , }
};
}
