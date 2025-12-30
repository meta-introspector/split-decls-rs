// Generated macro for try (macro)
macro_rules! Depcratetry {
() => {
// Module: crate
// Provides: {"try"}
// Dependencies: {}
# [doc = " Local macro to avoid `std::try!`, deprecated in Rust 1.39."] macro_rules ! try { ($ result : expr) => { match $ result { Ok (value) => value , Err (error) => return Err (error) , } } ; }
};
}
