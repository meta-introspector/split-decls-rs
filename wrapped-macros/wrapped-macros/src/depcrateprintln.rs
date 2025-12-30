// Generated macro for println (macro)
macro_rules! Depcrateprintln {
() => {
// Module: crate
// Provides: {"println"}
// Dependencies: {}
# [macro_export] macro_rules ! println { ($ lit : literal $ (, $ arg : expr) * $ (,) ?) => { { # [doc = " For binary size benchmarks we don't want to include `std::fmt::Write` machinery,"] # [doc = " which `println!` pulls in, but we do want to actually evaluate the arguments."] # [cfg (not (debug_assertions))] { struct Sink ; impl std :: fmt :: Write for Sink { fn write_str (& mut self , s : & str) -> Result < () , std :: fmt :: Error > { std :: hint :: black_box (s) ; Ok (()) } } $ (let _infallible = writeable :: Writeable :: write_to (&$ arg , & mut Sink) ;) * } # [cfg (debug_assertions)] { std :: println ! ($ lit , $ ($ arg ,) *) ; } } } ; }
};
}
