macro_rules! macro_139 {
    () => {
        cfg_if ! { if # [cfg (feature = "simd-accel")] { use :: core :: intrinsics :: unlikely ; use :: core :: intrinsics :: likely ; } else { # [inline (always)] fn unlikely (b : bool) -> bool { b } # [inline (always)] fn likely (b : bool) -> bool { b } } }
    };
}

macro_139!();