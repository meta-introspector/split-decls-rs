macro_rules! macro_159 {
    () => {
        cfg_if ! { if # [cfg (feature = "simd-accel")] { # [allow (unused_imports)] use :: core :: intrinsics :: unlikely ; # [allow (unused_imports)] use :: core :: intrinsics :: likely ; } else { # [allow (dead_code)] # [inline (always)] fn unlikely (b : bool) -> bool { b } # [allow (dead_code)] # [inline (always)] fn likely (b : bool) -> bool { b } } }
    };
}

macro_159!()