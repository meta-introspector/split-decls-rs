macro_rules! deps {
    () => {
        BytesToHexChars!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Iterator for BytesToHexChars < '_ > { type Item = char ; # [inline] fn next (& mut self) -> Option < Self :: Item > { match self . next . take () { Some (current) => Some (current) , None => self . inner . next () . map (| byte | { let current = self . table [(byte >> 4) as usize] as char ; self . next = Some (self . table [(byte & 0x0F) as usize] as char) ; current }) , } } fn size_hint (& self) -> (usize , Option < usize >) { let length = self . len () ; (length , Some (length)) } }
    };
}

impl_15!()