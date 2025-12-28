macro_rules! deps {
    () => {
        VecType!();
    };
}

macro_rules! normalize {
    () => {
        deps!();
        # [doc = " Normalize the integer, so any leading zero values are removed."] # [inline] pub fn normalize (x : & mut VecType) { while let Some (& value) = x . get (x . len () . wrapping_sub (1)) { if value == 0 { unsafe { x . set_len (x . len () - 1) } ; } else { break ; } } }
    };
}

normalize!()