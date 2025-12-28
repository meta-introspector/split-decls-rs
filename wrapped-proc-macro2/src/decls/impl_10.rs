macro_rules! deps {
    () => {
        Reject!();
        Cursor!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'a > Cursor < 'a > { pub (crate) fn advance (& self , bytes : usize) -> Cursor < 'a > { let (_front , rest) = self . rest . split_at (bytes) ; Cursor { rest , # [cfg (span_locations)] off : self . off + _front . chars () . count () as u32 , } } pub (crate) fn starts_with (& self , s : & str) -> bool { self . rest . starts_with (s) } pub (crate) fn starts_with_char (& self , ch : char) -> bool { self . rest . starts_with (ch) } pub (crate) fn starts_with_fn < Pattern > (& self , f : Pattern) -> bool where Pattern : FnMut (char) -> bool , { self . rest . starts_with (f) } pub (crate) fn is_empty (& self) -> bool { self . rest . is_empty () } fn len (& self) -> usize { self . rest . len () } fn as_bytes (& self) -> & 'a [u8] { self . rest . as_bytes () } fn bytes (& self) -> Bytes < 'a > { self . rest . bytes () } fn chars (& self) -> Chars < 'a > { self . rest . chars () } fn char_indices (& self) -> CharIndices < 'a > { self . rest . char_indices () } fn parse (& self , tag : & str) -> Result < Cursor < 'a > , Reject > { if self . starts_with (tag) { Ok (self . advance (tag . len ())) } else { Err (Reject) } } }
    };
}

impl_10!()