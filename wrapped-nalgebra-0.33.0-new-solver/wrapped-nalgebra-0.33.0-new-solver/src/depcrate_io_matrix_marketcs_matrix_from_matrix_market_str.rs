// Generated macro for cs_matrix_from_matrix_market_str (function)
macro_rules! Depcrate_io_matrix_marketcs_matrix_from_matrix_market_str {
() => {
// Module: crate::io::matrix_market
// Provides: {"cs_matrix_from_matrix_market_str"}
// Dependencies: {}
# [doc = " Parses a Matrix Market file described by the given string, and returns the corresponding sparse matrix."] pub fn cs_matrix_from_matrix_market_str < T : RealField > (data : & str) -> Option < CsMatrix < T > > { let file = MatrixMarketParser :: parse (Rule :: Document , data) . unwrap () . next () ? ; let mut shape = (0 , 0 , 0) ; let mut rows : Vec < usize > = Vec :: new () ; let mut cols : Vec < usize > = Vec :: new () ; let mut data : Vec < T > = Vec :: new () ; for line in file . into_inner () { match line . as_rule () { Rule :: Header => { } Rule :: Shape => { let mut inner = line . into_inner () ; shape . 0 = inner . next () ? . as_str () . parse :: < usize > () . ok () ? ; shape . 1 = inner . next () ? . as_str () . parse :: < usize > () . ok () ? ; shape . 2 = inner . next () ? . as_str () . parse :: < usize > () . ok () ? ; } Rule :: Entry => { let mut inner = line . into_inner () ; rows . push (inner . next () ? . as_str () . parse :: < usize > () . ok () ? - 1) ; cols . push (inner . next () ? . as_str () . parse :: < usize > () . ok () ? - 1) ; data . push (crate :: convert (inner . next () ? . as_str () . parse :: < f64 > () . ok () ?)) ; } _ => return None , } } Some (CsMatrix :: from_triplet (shape . 0 , shape . 1 , & rows , & cols , & data ,)) }
};
}
