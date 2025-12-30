// Generated macro for Splice (struct)
macro_rules! DepcrateSplice {
() => {
// Module: crate
// Provides: {"Splice"}
// Dependencies: {}
pub struct Splice < 'a , I : Iterator + 'a , const N : usize > { drain : Drain < 'a , I :: Item , N > , replace_with : I , }
};
}
