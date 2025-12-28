macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! nonzero {
    () => {
        deps!();
        # [doc = " Check if any of the remaining bits are non-zero."] # [inline] pub fn nonzero < T : Integer > (x : & [T] , rindex : usize) -> bool { let len = x . len () ; let slc = & x [.. len - rindex] ; slc . iter () . rev () . any (| & x | x != T :: ZERO) }
    };
}

nonzero!();