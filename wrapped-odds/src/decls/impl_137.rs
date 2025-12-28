macro_rules! deps {
    () => {
        CharChunks!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < 'a > Iterator for CharChunks < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { let s = self . s ; if s . is_empty () { return None ; } for (i , (j , ch)) in s . char_indices () . enumerate () { if i + 1 == self . n { let mid = j + ch . len_utf8 () ; let (part , tail) = (& s [.. mid] , & s [mid ..]) ; self . s = tail ; return Some (part) ; } } self . s = "" ; Some (s) } }
    };
}

impl_137!();