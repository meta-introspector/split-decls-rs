// Generated macro for drip (function)
macro_rules! Depcrate_destroydrip {
() => {
// Module: crate::destroy
// Provides: {"drip"}
// Dependencies: {}
# [doc = " Move a bunch of random pixels down one row."] # [doc = ""] # [doc = " Each pick some random pixels and move them each down one row. This is a very inefficient way to"] # [doc = " do this, but it works well enough for this demo."] # [expect (clippy :: cast_possible_truncation , clippy :: cast_precision_loss , clippy :: cast_sign_loss)] fn drip (frame_count : usize , area : Rect , buf : & mut Buffer) { let mut rng = rand_chacha :: ChaCha8Rng :: seed_from_u64 (10) ; let ramp_frames = 450 ; let fractional_speed = frame_count as f64 / f64 :: from (ramp_frames) ; let variable_speed = DRIP_SPEED as f64 * fractional_speed * fractional_speed * fractional_speed ; let pixel_count = (frame_count as f64 * variable_speed) . floor () as usize ; for _ in 0 .. pixel_count { let src_x = rng . random_range (0 .. area . width) ; let src_y = rng . random_range (1 .. area . height - 2) ; let src = buf [(src_x , src_y)] . clone () ; if rng . random_ratio (1 , 100) { let dest_x = rng . random_range (src_x . saturating_sub (5) .. src_x . saturating_add (5)) . clamp (area . left () , area . right () - 1) ; let dest_y = area . top () + 1 ; let dest = & mut buf [(dest_x , dest_y)] ; if rng . random_ratio (1 , 10) { * dest = src ; } else { dest . reset () ; } } else { let dest_x = src_x ; let dest_y = src_y . saturating_add (1) . min (area . bottom () - 2) ; buf [(dest_x , dest_y)] = src ; } } }
};
}
