macro_rules! deps {
    () => {
        Result!();
        Bytes!();
        FunctionStartsIterator!();
    };
}

macro_rules! impl_552 {
    () => {
        deps!();
        impl < 'data > FunctionStartsIterator < 'data > { # [doc = " Returns the next function start address."] pub fn next (& mut self) -> Result < Option < u64 > > { if self . data . is_empty () { return Ok (None) ; } let result = self . parse () ; if result . is_err () { self . data = Bytes (& []) ; } result } fn parse (& mut self) -> Result < Option < u64 > > { let delta = self . data . read_uleb128 () . read_error ("Invalid ULEB128 in LC_FUNCTION_STARTS") ? ; if delta == 0 { self . data = Bytes (& []) ; return Ok (None) ; } self . addr = self . addr . checked_add (delta) . read_error ("Address overflow in LC_FUNCTION_STARTS") ? ; Ok (Some (self . addr)) } }
    };
}

impl_552!()