macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! base128_len {
    () => {
        deps!();
        # [doc = " Compute the length of an arc when encoded in base 128."] const fn base128_len (arc : Arc) -> usize { match arc { 0 ..= 0x7f => 1 , 0x80 ..= 0x3fff => 2 , 0x4000 ..= 0x1fffff => 3 , 0x200000 ..= 0x0fffffff => 4 , _ => 5 , } }
    };
}

base128_len!();