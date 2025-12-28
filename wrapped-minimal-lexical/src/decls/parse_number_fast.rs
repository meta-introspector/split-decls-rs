macro_rules! deps {
    () => {
        Number!();
    };
}

macro_rules! parse_number_fast {
    () => {
        deps!();
        # [doc = " Try to parse the significant digits quickly."] # [doc = ""] # [doc = " This attempts a very quick parse, to deal with common cases."] # [doc = ""] # [doc = " * `integer`     - Slice containing the integer digits."] # [doc = " * `fraction`    - Slice containing the fraction digits."] # [inline] fn parse_number_fast < 'a , Iter1 , Iter2 > (integer : Iter1 , fraction : Iter2 , exponent : i32 ,) -> Option < Number > where Iter1 : Iterator < Item = & 'a u8 > , Iter2 : Iterator < Item = & 'a u8 > , { let mut num = Number :: default () ; let mut integer_count : usize = 0 ; let mut fraction_count : usize = 0 ; for & c in integer { integer_count += 1 ; let digit = c - b'0' ; num . mantissa = num . mantissa . wrapping_mul (10) . wrapping_add (digit as u64) ; } for & c in fraction { fraction_count += 1 ; let digit = c - b'0' ; num . mantissa = num . mantissa . wrapping_mul (10) . wrapping_add (digit as u64) ; } if integer_count + fraction_count <= 19 { num . exponent = exponent . saturating_sub (fraction_count as i32) ; Some (num) } else { None } }
    };
}

parse_number_fast!()