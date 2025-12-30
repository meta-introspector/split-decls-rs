// Generated macro for shoes_in_size (function)
macro_rules! Depcrateshoes_in_size {
() => {
// Module: crate
// Provides: {"shoes_in_size"}
// Dependencies: {}
fn shoes_in_size (shoes : Vec < Shoe > , shoe_size : u32) -> Vec < Shoe > { shoes . into_iter () . filter (| s | s . size == shoe_size) . collect () }
};
}
