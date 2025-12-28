macro_rules! deps {
    () => {
        Read!();
        Value!();
        Position!();
        Reference!();
        BorrowedRawDeserializer!();
        Result!();
        ErrorCode!();
        SliceRead!();
    };
}

macro_rules! impl_581 {
    () => {
        deps!();
        impl < 'a > Read < 'a > for SliceRead < 'a > { # [inline] fn next (& mut self) -> Result < Option < u8 > > { Ok (if self . index < self . slice . len () { let ch = self . slice [self . index] ; self . index += 1 ; Some (ch) } else { None }) } # [inline] fn peek (& mut self) -> Result < Option < u8 > > { Ok (if self . index < self . slice . len () { Some (self . slice [self . index]) } else { None }) } # [inline] fn discard (& mut self) { self . index += 1 ; } fn position (& self) -> Position { self . position_of_index (self . index) } fn peek_position (& self) -> Position { self . position_of_index (cmp :: min (self . slice . len () , self . index + 1)) } fn byte_offset (& self) -> usize { self . index } fn parse_str < 's > (& 's mut self , scratch : & 's mut Vec < u8 >) -> Result < Reference < 'a , 's , str > > { self . parse_str_bytes (scratch , true , as_str) } fn parse_str_raw < 's > (& 's mut self , scratch : & 's mut Vec < u8 > ,) -> Result < Reference < 'a , 's , [u8] > > { self . parse_str_bytes (scratch , false , | _ , bytes | Ok (bytes)) } fn ignore_str (& mut self) -> Result < () > { loop { self . skip_to_escape (true) ; if self . index == self . slice . len () { return error (self , ErrorCode :: EofWhileParsingString) ; } match self . slice [self . index] { b'"' => { self . index += 1 ; return Ok (()) ; } b'\\' => { self . index += 1 ; tri ! (ignore_escape (self)) ; } _ => { return error (self , ErrorCode :: ControlCharacterWhileParsingString) ; } } } } # [inline] fn decode_hex_escape (& mut self) -> Result < u16 > { match self . slice [self . index ..] { [a , b , c , d , ..] => { self . index += 4 ; match decode_four_hex_digits (a , b , c , d) { Some (val) => Ok (val) , None => error (self , ErrorCode :: InvalidEscape) , } } _ => { self . index = self . slice . len () ; error (self , ErrorCode :: EofWhileParsingString) } } } # [cfg (feature = "raw_value")] fn begin_raw_buffering (& mut self) { self . raw_buffering_start_index = self . index ; } # [cfg (feature = "raw_value")] fn end_raw_buffering < V > (& mut self , visitor : V) -> Result < V :: Value > where V : Visitor < 'a > , { let raw = & self . slice [self . raw_buffering_start_index .. self . index] ; let raw = match str :: from_utf8 (raw) { Ok (raw) => raw , Err (_) => return error (self , ErrorCode :: InvalidUnicodeCodePoint) , } ; visitor . visit_map (BorrowedRawDeserializer { raw_value : Some (raw) , }) } const should_early_return_if_failed : bool = false ; # [inline] # [cold] fn set_failed (& mut self , _failed : & mut bool) { self . slice = & self . slice [.. self . index] ; } }
    };
}

impl_581!();