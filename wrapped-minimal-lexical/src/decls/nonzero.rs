macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! nonzero {
    () => {
        deps!();
        # [doc = " Check if any of the remaining bits are non-zero."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Safe as long as `rindex <= x.len()`."] # [inline] pub fn nonzero (x : & [Limb] , rindex : usize) -> bool { debug_assert ! (rindex <= x . len ()) ; let len = x . len () ; let slc = & x [.. len - rindex] ; slc . iter () . rev () . any (| & x | x != 0) }
    };
}

nonzero!();