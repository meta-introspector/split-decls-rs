// Generated macro for t (macro)
macro_rules! Depcratet {
() => {
// Module: crate
// Provides: {"t"}
// Dependencies: {}
# [doc = " Unwrap a `Result` with a useful panic message"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use cargo_test_support::t;"] # [doc = " t!(std::fs::read_to_string(\"Cargo.toml\"));"] # [doc = " ```"] # [macro_export] macro_rules ! t { ($ e : expr) => { match $ e { Ok (e) => e , Err (e) => $ crate :: panic_error (& format ! ("failed running {}" , stringify ! ($ e)) , e) , } } ; }
};
}
