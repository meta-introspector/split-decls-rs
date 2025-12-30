// Generated macro for from_str_generic (function)
macro_rules! Depcratefrom_str_generic {
() => {
// Module: crate
// Provides: {"from_str_generic"}
// Dependencies: {}
fn from_str_generic < T , E , F > (s : & str , from : F) -> Result < Complex < T > , ParseComplexError < E > > where F : Fn (& str) -> Result < T , E > , T : Clone + Num , { let imag = match s . rfind ('j') { None => 'i' , _ => 'j' , } ; let mut neg_b = false ; let mut a = s ; let mut b = "" ; for (i , w) in s . as_bytes () . windows (2) . enumerate () { let p = w [0] ; let c = w [1] ; if (c == b'+' || c == b'-') && ! (p == b'e' || p == b'E') { a = s [..= i] . trim_end_matches (char :: is_whitespace) ; b = s [i + 2 ..] . trim_start_matches (char :: is_whitespace) ; neg_b = c == b'-' ; if b . is_empty () || (neg_b && b . starts_with ('-')) { return Err (ParseComplexError :: expr_error ()) ; } break ; } } if b . is_empty () { b = if a . ends_with (imag) { "0" } else { "0i" } ; } let re ; let neg_re ; let im ; let neg_im ; if a . ends_with (imag) { im = a ; neg_im = false ; re = b ; neg_re = neg_b ; } else if b . ends_with (imag) { re = a ; neg_re = false ; im = b ; neg_im = neg_b ; } else { return Err (ParseComplexError :: expr_error ()) ; } let re = from (re) . map_err (ParseComplexError :: from_error) ? ; let re = if neg_re { T :: zero () - re } else { re } ; let mut im = & im [.. im . len () - 1] ; if im . is_empty () || im == "+" { im = "1" ; } else if im == "-" { im = "-1" ; } let im = from (im) . map_err (ParseComplexError :: from_error) ? ; let im = if neg_im { T :: zero () - im } else { im } ; Ok (Complex :: new (re , im)) }
};
}
