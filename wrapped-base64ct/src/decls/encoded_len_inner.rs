macro_rules! encoded_len_inner {
    () => {
        # [allow (clippy :: arithmetic_side_effects)] # [inline (always)] const fn encoded_len_inner (n : usize , padded : bool) -> Option < usize > { match n . checked_mul (4) { Some (q) => { if padded { Some (((q / 3) + 3) & ! 3) } else { Some ((q / 3) + (q % 3 != 0) as usize) } } None => None , } }
    };
}

encoded_len_inner!()