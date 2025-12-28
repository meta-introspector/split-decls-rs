macro_rules! deps {
    () => {
        ByteClassElementRanges!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl < 'a > Iterator for ByteClassElementRanges < 'a > { type Item = (u8 , u8) ; fn next (& mut self) -> Option < (u8 , u8) > { loop { let element = match self . elements . next () { None => return self . range . take () , Some (element) => element , } ; match self . range . take () { None => { self . range = Some ((element , element)) ; } Some ((start , end)) => { if usize :: from (end) + 1 != usize :: from (element) { self . range = Some ((element , element)) ; return Some ((start , end)) ; } self . range = Some ((start , element)) ; } } } } }
    };
}

impl_319!();