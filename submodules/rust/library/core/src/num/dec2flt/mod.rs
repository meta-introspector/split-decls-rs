mkuse!{use self :: common :: BiasedFp ;}
mkuse!{use self :: float :: RawFloat ;}
mkuse!{use self :: lemire :: compute_float ;}
mkuse!{use self :: parse :: { parse_inf_nan , parse_number } ;}
mkuse!{use self :: slow :: parse_long_mantissa ;}
mkuse!{use crate :: error :: Error ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: str :: FromStr ;}
mkmod!{common, { 
                getname!(common);
                getsrc!(common);
                getpath!(common);
                get_deps!(common);
                get_crates!(common);
                mkinclude!(common);
                 
            }}
mkmod!{decimal, { 
                getname!(decimal);
                getsrc!(decimal);
                getpath!(decimal);
                get_deps!(decimal);
                get_crates!(decimal);
                mkinclude!(decimal);
                 
            }}
mkmod!{decimal_seq, { 
                getname!(decimal_seq);
                getsrc!(decimal_seq);
                getpath!(decimal_seq);
                get_deps!(decimal_seq);
                get_crates!(decimal_seq);
                mkinclude!(decimal_seq);
                 
            }}
mkmod!{fpu, { 
                getname!(fpu);
                getsrc!(fpu);
                getpath!(fpu);
                get_deps!(fpu);
                get_crates!(fpu);
                mkinclude!(fpu);
                 
            }}
mkmod!{slow, { 
                getname!(slow);
                getsrc!(slow);
                getpath!(slow);
                get_deps!(slow);
                get_crates!(slow);
                mkinclude!(slow);
                 
            }}
mkmod!{table, { 
                getname!(table);
                getsrc!(table);
                getpath!(table);
                get_deps!(table);
                get_crates!(table);
                mkinclude!(table);
                 
            }}
mkmod!{float, { 
                getname!(float);
                getsrc!(float);
                getpath!(float);
                get_deps!(float);
                get_crates!(float);
                mkinclude!(float);
                 
            }}
mkmod!{lemire, { 
                getname!(lemire);
                getsrc!(lemire);
                getpath!(lemire);
                get_deps!(lemire);
                get_crates!(lemire);
                mkinclude!(lemire);
                 
            }}
mkmod!{parse, { 
                getname!(parse);
                getsrc!(parse);
                getpath!(parse);
                get_deps!(parse);
                get_crates!(parse);
                mkinclude!(parse);
                 
            }}
mkitem!{macro_rules ! from_str_float_impl { ($ t : ty) => { # [stable (feature = "rust1" , since = "1.0.0")] impl FromStr for $ t { type Err = ParseFloatError ; # [doc = " Converts a string in base 10 to a float."] # [doc = " Accepts an optional decimal exponent."] # [doc = ""] # [doc = " This function accepts strings such as"] # [doc = ""] # [doc = " * '3.14'"] # [doc = " * '-3.14'"] # [doc = " * '2.5E10', or equivalently, '2.5e10'"] # [doc = " * '2.5E-10'"] # [doc = " * '5.'"] # [doc = " * '.5', or, equivalently, '0.5'"] # [doc = " * '7'"] # [doc = " * '007'"] # [doc = " * 'inf', '-inf', '+infinity', 'NaN'"] # [doc = ""] # [doc = " Note that alphabetical characters are not case-sensitive."] # [doc = ""] # [doc = " Leading and trailing whitespace represent an error."] # [doc = ""] # [doc = " # Grammar"] # [doc = ""] # [doc = " All strings that adhere to the following [EBNF] grammar when"] # [doc = " lowercased will result in an [`Ok`] being returned:"] # [doc = ""] # [doc = " ```txt"] # [doc = " Float  ::= Sign? ( 'inf' | 'infinity' | 'nan' | Number )"] # [doc = " Number ::= ( Digit+ |"] # [doc = "              Digit+ '.' Digit* |"] # [doc = "              Digit* '.' Digit+ ) Exp?"] # [doc = " Exp    ::= 'e' Sign? Digit+"] # [doc = " Sign   ::= [+-]"] # [doc = " Digit  ::= [0-9]"] # [doc = " ```"] # [doc = ""] # [doc = " [EBNF]: https://www.w3.org/TR/REC-xml/#sec-notation"] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * src - A string"] # [doc = ""] # [doc = " # Return value"] # [doc = ""] # [doc = " `Err(ParseFloatError)` if the string did not represent a valid"] # [doc = " number. Otherwise, `Ok(n)` where `n` is the closest"] # [doc = " representable floating-point number to the number represented"] # [doc = " by `src` (following the same rules for rounding as for the"] # [doc = " results of primitive operations)."] # [inline (never)] fn from_str (src : & str) -> Result < Self , ParseFloatError > { dec2flt (src) } } } ; }}
mkitem!{# [cfg (target_has_reliable_f16)] from_str_float_impl ! (f16) ;}
mkitem!{from_str_float_impl ! (f32) ;}
mkitem!{from_str_float_impl ! (f64) ;}
mkitem!{mkimpl!{# [cfg (not (target_has_reliable_f16))] impl FromStr for f16 { type Err = ParseFloatError ; # [inline] fn from_str (_src : & str) -> Result < Self , ParseFloatError > { unimplemented ! ("requires target_has_reliable_f16") } }}}
mkitem!{mkstruct!{# [doc = " An error which can be returned when parsing a float."] # [doc = ""] # [doc = " This error is used as the error type for the [`FromStr`] implementation"] # [doc = " for [`f32`] and [`f64`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::str::FromStr;"] # [doc = ""] # [doc = " if let Err(e) = f64::from_str(\"a.12\") {"] # [doc = "     println!(\"Failed conversion to f64: {e}\");"] # [doc = " }"] # [doc = " ```"] # [derive (Debug , Clone , PartialEq , Eq)] # [stable (feature = "rust1" , since = "1.0.0")] pub struct ParseFloatError { kind : FloatErrorKind , }}}
mkitem!{mkenum!{# [derive (Debug , Clone , PartialEq , Eq)] enum FloatErrorKind { Empty , Invalid , }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl Error for ParseFloatError { }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl fmt :: Display for ParseFloatError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . kind { FloatErrorKind :: Empty => "cannot parse float from empty string" , FloatErrorKind :: Invalid => "invalid float literal" , } . fmt (f) } }}}

macro_rules! pfe_empty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pfe_empty in module {}", module_path!());
    };
}

mkfn!{
    pfe_empty_introspect!();
    # [inline] pub (super) fn pfe_empty () -> ParseFloatError { ParseFloatError { kind : FloatErrorKind :: Empty } }
}

macro_rules! pfe_invalid_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pfe_invalid in module {}", module_path!());
    };
}

mkfn!{
    pfe_invalid_introspect!();
    # [inline] pub fn pfe_invalid () -> ParseFloatError { ParseFloatError { kind : FloatErrorKind :: Invalid } }
}

macro_rules! biased_fp_to_float_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function biased_fp_to_float in module {}", module_path!());
    };
}

mkfn!{
    biased_fp_to_float_introspect!();
    # [doc = " Converts a `BiasedFp` to the closest machine float type."] fn biased_fp_to_float < F : RawFloat > (x : BiasedFp) -> F { let mut word = x . m ; word |= (x . p_biased as u64) << F :: SIG_BITS ; F :: from_u64_bits (word) }
}

macro_rules! dec2flt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dec2flt in module {}", module_path!());
    };
}

mkfn!{
    dec2flt_introspect!();
    # [doc = " Converts a decimal string into a floating point number."] # [inline (always)] pub fn dec2flt < F : RawFloat > (s : & str) -> Result < F , ParseFloatError > { let mut s = s . as_bytes () ; let c = if let Some (& c) = s . first () { c } else { return Err (pfe_empty ()) ; } ; let negative = c == b'-' ; if c == b'-' || c == b'+' { s = & s [1 ..] ; } if s . is_empty () { return Err (pfe_invalid ()) ; } let mut num = match parse_number (s) { Some (r) => r , None if let Some (value) = parse_inf_nan (s , negative) => return Ok (value) , None => return Err (pfe_invalid ()) , } ; num . negative = negative ; if ! cfg ! (feature = "optimize_for_size") { if let Some (value) = num . try_fast_path :: < F > () { return Ok (value) ; } } let mut fp = compute_float :: < F > (num . exponent , num . mantissa) ; if num . many_digits && fp . p_biased >= 0 && fp != compute_float :: < F > (num . exponent , num . mantissa + 1) { fp . p_biased = - 1 ; } if fp . p_biased < 0 { fp = parse_long_mantissa :: < F > (s) ; } let mut float = biased_fp_to_float :: < F > (fp) ; if num . negative { float = - float ; } Ok (float) }
}