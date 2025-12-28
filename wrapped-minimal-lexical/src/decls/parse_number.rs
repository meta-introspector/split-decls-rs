macro_rules! deps {
    () => {
        Number!();
    };
}

macro_rules! parse_number {
    () => {
        deps!();
        # [doc = " Parse the significant digits of the float and adjust the exponent."] # [doc = ""] # [doc = " * `integer`     - Slice containing the integer digits."] # [doc = " * `fraction`    - Slice containing the fraction digits."] # [inline] fn parse_number < 'a , Iter1 , Iter2 > (mut integer : Iter1 , mut fraction : Iter2 , exponent : i32) -> Number where Iter1 : Iterator < Item = & 'a u8 > + Clone , Iter2 : Iterator < Item = & 'a u8 > + Clone , { if let Some (num) = parse_number_fast (integer . clone () , fraction . clone () , exponent) { return num ; } let mut num = Number :: default () ; let mut count = 0 ; while let Some (& c) = integer . next () { count += 1 ; if count == 20 { num . many_digits = true ; num . exponent = exponent . saturating_add (into_i32 (1 + integer . count ())) ; return num ; } else { let digit = c - b'0' ; num . mantissa = num . mantissa * 10 + digit as u64 ; } } let mut fraction_count : usize = 0 ; if count == 0 { for & c in & mut fraction { fraction_count += 1 ; if c != b'0' { count += 1 ; let digit = c - b'0' ; num . mantissa = num . mantissa * 10 + digit as u64 ; break ; } } } for c in fraction { fraction_count += 1 ; count += 1 ; if count == 20 { num . many_digits = true ; num . exponent = exponent . saturating_sub (fraction_count as i32 - 1) ; return num ; } else { let digit = c - b'0' ; num . mantissa = num . mantissa * 10 + digit as u64 ; } } num . exponent = exponent . saturating_sub (fraction_count as i32) ; num }
    };
}

parse_number!();