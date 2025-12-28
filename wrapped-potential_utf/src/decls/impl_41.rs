macro_rules! deps {
    () => {
        PotentialUtf16!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        # [doc = " This impl requires enabling the optional `writeable` Cargo feature"] impl TryWriteable for & '_ PotentialUtf16 { type Error = DecodeUtf16Error ; fn try_write_to_parts < S : PartsWrite + ? Sized > (& self , sink : & mut S ,) -> Result < Result < () , Self :: Error > , fmt :: Error > { let mut r = Ok (()) ; for c in core :: char :: decode_utf16 (self . 0 . iter () . copied ()) { match c { Ok (c) => sink . write_char (c) ? , Err (e) => { if r . is_ok () { r = Err (e) ; } sink . with_part (Part :: ERROR , | s | s . write_char (char :: REPLACEMENT_CHARACTER)) ? ; } } } Ok (r) } fn writeable_length_hint (& self) -> LengthHint { LengthHint :: between (self . 0 . len () , self . 0 . len () * 3) } }
    };
}

impl_41!()