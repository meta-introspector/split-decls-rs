macro_rules! deps {
    () => {
        ParseAlphabetError!();
        Alphabet!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl Alphabet { # [doc = " Performs no checks so that it can be const."] # [doc = " Used only for known-valid strings."] const fn from_str_unchecked (alphabet : & str) -> Self { let mut symbols = [0_u8 ; ALPHABET_SIZE] ; let source_bytes = alphabet . as_bytes () ; let mut index = 0 ; while index < ALPHABET_SIZE { symbols [index] = source_bytes [index] ; index += 1 ; } Self { symbols } } # [doc = " Create an `Alphabet` from a string of 64 unique printable ASCII bytes."] # [doc = ""] # [doc = " The `=` byte is not allowed as it is used for padding."] pub const fn new (alphabet : & str) -> Result < Self , ParseAlphabetError > { let bytes = alphabet . as_bytes () ; if bytes . len () != ALPHABET_SIZE { return Err (ParseAlphabetError :: InvalidLength) ; } { let mut index = 0 ; while index < ALPHABET_SIZE { let byte = bytes [index] ; if ! (byte >= 32_u8 && byte <= 126_u8) { return Err (ParseAlphabetError :: UnprintableByte (byte)) ; } if byte == PAD_BYTE { return Err (ParseAlphabetError :: ReservedByte (byte)) ; } let mut probe_index = 0 ; while probe_index < ALPHABET_SIZE { if probe_index == index { probe_index += 1 ; continue ; } let probe_byte = bytes [probe_index] ; if byte == probe_byte { return Err (ParseAlphabetError :: DuplicatedByte (byte)) ; } probe_index += 1 ; } index += 1 ; } } Ok (Self :: from_str_unchecked (alphabet)) } # [doc = " Create a `&str` from the symbols in the `Alphabet`"] # [must_use] pub fn as_str (& self) -> & str { core :: str :: from_utf8 (& self . symbols) . unwrap () } }
    };
}

impl_205!()