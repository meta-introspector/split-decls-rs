macro_rules! shuffle {
    () => {
        # [inline (always)] pub (crate) fn shuffle (a : u128) -> u128 { # [cfg (all (target_feature = "ssse3" , not (miri)))] { # [cfg (target_arch = "x86")] use core :: arch :: x86 :: * ; # [cfg (target_arch = "x86_64")] use core :: arch :: x86_64 :: * ; unsafe { transmute ! (_mm_shuffle_epi8 (transmute ! (a) , transmute ! (SHUFFLE_MASK))) } } # [cfg (not (all (target_feature = "ssse3" , not (miri))))] { a . swap_bytes () } }
    };
}

shuffle!();