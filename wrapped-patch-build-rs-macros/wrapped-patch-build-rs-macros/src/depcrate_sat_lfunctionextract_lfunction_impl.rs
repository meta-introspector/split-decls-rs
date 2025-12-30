// Generated macro for extract_lfunction_impl (function)
macro_rules! Depcrate_sat_lfunctionextract_lfunction_impl {
() => {
// Module: crate::sat_lfunction
// Provides: {"extract_lfunction_impl"}
// Dependencies: {}
# [decl2 (fn , name = "extract_lfunction_impl" , vis = "pub" , hash = "ea5943b4")] pub fn extract_lfunction_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let rust_vector = input_str . value () ; quote ! { { println ! ("cargo:warning=📊 Extracting L-function from rustc vector") ; let rust_nums : Vec < f64 > = # rust_vector . split (',') . filter_map (| s | s . trim () . parse () . ok ()) . collect () ; let mut l_coeffs = Vec :: new () ; for (n , & a_n) in rust_nums . iter () . enumerate () { if n > 0 { let coeff = a_n / (n as f64) . powf (0.5) ; l_coeffs . push (coeff) ; } } let l_signature = l_coeffs . iter () . take (10) . map (| x | format ! ("{:.3}" , x)) . collect ::< Vec < _ >> () . join (",") ; let lfunction = format ! ("L_rustc(s) = Σ a_n/n^s where a_n = [{}...] (first 10 coefficients)" , l_signature) ; println ! ("cargo:warning=∞ L-function extracted: L_rustc(s)") ; lfunction } } . into () }
};
}
