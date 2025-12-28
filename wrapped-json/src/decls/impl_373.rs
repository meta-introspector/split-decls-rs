macro_rules! deps {
    () => {
        Result!();
        LineColIterator!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        impl < I > Iterator for LineColIterator < I > where I : Iterator < Item = io :: Result < u8 > > , { type Item = io :: Result < u8 > ; fn next (& mut self) -> Option < io :: Result < u8 > > { match self . iter . next () { None => None , Some (Ok (b'\n')) => { self . start_of_line += self . col + 1 ; self . line += 1 ; self . col = 0 ; Some (Ok (b'\n')) } Some (Ok (c)) => { self . col += 1 ; Some (Ok (c)) } Some (Err (e)) => Some (Err (e)) , } } }
    };
}

impl_373!();