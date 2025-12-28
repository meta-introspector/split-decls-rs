macro_rules! deps {
    () => {
        ParseResult!();
    };
}

macro_rules! number {
    () => {
        deps!();
        # [doc = " Tries to parse the non-negative number from `min` to `max` digits."] # [doc = ""] # [doc = " The absence of digits at all is an unconditional error."] # [doc = " More than `max` digits are consumed up to the first `max` digits."] # [doc = " Any number that does not fit in `i64` is an error."] # [inline] pub (super) fn number (s : & str , min : usize , max : usize) -> ParseResult < (& str , i64) > { assert ! (min <= max) ; let bytes = s . as_bytes () ; if bytes . len () < min { return Err (TOO_SHORT) ; } let mut n = 0i64 ; for (i , c) in bytes . iter () . take (max) . cloned () . enumerate () { if ! c . is_ascii_digit () { if i < min { return Err (INVALID) ; } else { return Ok ((& s [i ..] , n)) ; } } n = match n . checked_mul (10) . and_then (| n | n . checked_add ((c - b'0') as i64)) { Some (n) => n , None => return Err (OUT_OF_RANGE) , } ; } Ok ((& s [core :: cmp :: min (max , bytes . len ()) ..] , n)) }
    };
}

number!()