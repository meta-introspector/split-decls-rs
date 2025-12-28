macro_rules! deps {
    () => {
        LimbBuffer!();
    };
}

macro_rules! wnaf_form {
    () => {
        deps!();
        # [doc = " Replaces the contents of `wnaf` with the w-NAF representation of a little-endian"] # [doc = " scalar."] pub (crate) fn wnaf_form < S : AsRef < [u8] > > (wnaf : & mut Vec < i64 > , c : S , window : usize) { debug_assert ! (window >= 2) ; debug_assert ! (window <= 64) ; let bit_len = c . as_ref () . len () * 8 ; wnaf . truncate (0) ; wnaf . reserve (bit_len) ; let mut limbs = LimbBuffer :: new (c . as_ref ()) ; let width = 1u64 << window ; let window_mask = width - 1 ; let mut pos = 0 ; let mut carry = 0 ; while pos < bit_len { let u64_idx = pos / 64 ; let bit_idx = pos % 64 ; let (cur_u64 , next_u64) = limbs . get (u64_idx) ; let bit_buf = if bit_idx + window < 64 { cur_u64 >> bit_idx } else { (cur_u64 >> bit_idx) | (next_u64 << (64 - bit_idx)) } ; let window_val = carry + (bit_buf & window_mask) ; if window_val & 1 == 0 { wnaf . push (0) ; pos += 1 ; } else { wnaf . push (if window_val < width / 2 { carry = 0 ; window_val as i64 } else { carry = 1 ; (window_val as i64) . wrapping_sub (width as i64) }) ; wnaf . extend (iter :: repeat (0) . take (window - 1)) ; pos += window ; } } }
    };
}

wnaf_form!();