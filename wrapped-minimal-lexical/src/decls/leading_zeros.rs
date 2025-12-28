macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! leading_zeros {
    () => {
        deps!();
        # [doc = " Get number of leading zero bits in the storage."] # [inline] pub fn leading_zeros (x : & [Limb]) -> u32 { let length = x . len () ; if let Some (& value) = x . get (length . wrapping_sub (1)) { value . leading_zeros () } else { 0 } }
    };
}

leading_zeros!()