macro_rules! deps {
    () => {
        Position!();
        BorrowedRawDeserializer!();
        Read!();
        StrRead!();
        Result!();
        Value!();
        Reference!();
    };
}

macro_rules! impl_584 {
    () => {
        deps!();
        impl < 'a > Read < 'a > for StrRead < 'a > { # [inline] fn next (& mut self) -> Result < Option < u8 > > { self . delegate . next () } # [inline] fn peek (& mut self) -> Result < Option < u8 > > { self . delegate . peek () } # [inline] fn discard (& mut self) { self . delegate . discard () ; } fn position (& self) -> Position { self . delegate . position () } fn peek_position (& self) -> Position { self . delegate . peek_position () } fn byte_offset (& self) -> usize { self . delegate . byte_offset () } fn parse_str < 's > (& 's mut self , scratch : & 's mut Vec < u8 >) -> Result < Reference < 'a , 's , str > > { self . delegate . parse_str_bytes (scratch , true , | _ , bytes | { Ok (unsafe { str :: from_utf8_unchecked (bytes) }) }) } fn parse_str_raw < 's > (& 's mut self , scratch : & 's mut Vec < u8 > ,) -> Result < Reference < 'a , 's , [u8] > > { self . delegate . parse_str_raw (scratch) } fn ignore_str (& mut self) -> Result < () > { self . delegate . ignore_str () } fn decode_hex_escape (& mut self) -> Result < u16 > { self . delegate . decode_hex_escape () } # [cfg (feature = "raw_value")] fn begin_raw_buffering (& mut self) { self . delegate . begin_raw_buffering () ; } # [cfg (feature = "raw_value")] fn end_raw_buffering < V > (& mut self , visitor : V) -> Result < V :: Value > where V : Visitor < 'a > , { let raw = & self . data [self . delegate . raw_buffering_start_index .. self . delegate . index] ; visitor . visit_map (BorrowedRawDeserializer { raw_value : Some (raw) , }) } const should_early_return_if_failed : bool = false ; # [inline] # [cold] fn set_failed (& mut self , failed : & mut bool) { self . delegate . set_failed (failed) ; } }
    };
}

impl_584!();