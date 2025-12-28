macro_rules! deps {
    () => {
        PunycodeCodeUnit!();
        PunycodeCaller!();
        Decode!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T : PunycodeCodeUnit + Copy , C : PunycodeCaller > Iterator for Decode < '_ , T , C > { type Item = char ; fn next (& mut self) -> Option < Self :: Item > { loop { match self . insertions . get (self . inserted) { Some ((pos , c)) if * pos == self . position => { self . inserted += 1 ; self . position += 1 ; return Some (* c) ; } _ => { } } if let Some (c) = self . base . next () { self . position += 1 ; return Some (if C :: EXTERNAL_CALLER { c . char () } else { c . char_ascii_lower_case () }) ; } else if self . inserted >= self . insertions . len () { return None ; } } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len - self . position ; (len , Some (len)) } }
    };
}

impl_32!()