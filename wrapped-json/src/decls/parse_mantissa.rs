macro_rules! deps {
    () => {
        Float!();
        Bigint!();
        Limb!();
    };
}

macro_rules! parse_mantissa {
    () => {
        deps!();
        # [doc = " Parse the full mantissa into a big integer."] # [doc = ""] # [doc = " Max digits is the maximum number of digits plus one."] fn parse_mantissa < F > (integer : & [u8] , fraction : & [u8]) -> Bigint where F : Float , { let small_powers = POW10_LIMB ; let step = small_powers . len () - 2 ; let max_digits = F :: MAX_DIGITS - 1 ; let mut counter = 0 ; let mut value : Limb = 0 ; let mut i : usize = 0 ; let mut result = Bigint :: default () ; for & digit in integer . iter () . chain (fraction) { if counter == step { result . imul_small (small_powers [counter]) ; result . iadd_small (value) ; counter = 0 ; value = 0 ; } value *= 10 ; value += as_limb (to_digit (digit) . unwrap ()) ; i += 1 ; counter += 1 ; if i == max_digits { break ; } } if counter != 0 { result . imul_small (small_powers [counter]) ; result . iadd_small (value) ; } if i < integer . len () + fraction . len () { result . imul_small (10) ; result . iadd_small (1) ; } result }
    };
}

parse_mantissa!();