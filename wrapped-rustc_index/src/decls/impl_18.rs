macro_rules! deps {
    () => {
        Idx!();
        DenseBitSet!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < T : Idx > ToString for DenseBitSet < T > { fn to_string (& self) -> String { let mut result = String :: new () ; let mut sep = '[' ; let mut i = 0 ; for word in & self . words { let mut word = * word ; for _ in 0 .. WORD_BYTES { let remain = self . domain_size - i ; let mask = if remain <= 8 { (1 << remain) - 1 } else { 0xFF } ; assert ! (mask <= 0xFF) ; let byte = word & mask ; result . push_str (& format ! ("{sep}{byte:02x}")) ; if remain <= 8 { break ; } word >>= 8 ; i += 8 ; sep = '-' ; } sep = '|' ; } result . push (']') ; result } }
    };
}

impl_18!()