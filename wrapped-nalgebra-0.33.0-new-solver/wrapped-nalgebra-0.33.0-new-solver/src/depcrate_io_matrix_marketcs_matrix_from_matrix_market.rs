// Generated macro for cs_matrix_from_matrix_market (function)
macro_rules! Depcrate_io_matrix_marketcs_matrix_from_matrix_market {
() => {
// Module: crate::io::matrix_market
// Provides: {"cs_matrix_from_matrix_market"}
// Dependencies: {}
# [doc = " Parses a Matrix Market file at the given path, and returns the corresponding sparse matrix."] pub fn cs_matrix_from_matrix_market < T : RealField , P : AsRef < Path > > (path : P) -> Option < CsMatrix < T > > { let file = fs :: read_to_string (path) . ok () ? ; cs_matrix_from_matrix_market_str (& file) }
};
}
