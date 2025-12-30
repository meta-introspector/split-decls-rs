// Generated macro for dur2ms (function)
macro_rules! Depcratedur2ms {
() => {
// Module: crate
// Provides: {"dur2ms"}
// Dependencies: {}
fn dur2ms (dur : Option < Duration >) -> u32 { let dur = match dur { Some (dur) => dur , None => return INFINITE , } ; let ms = dur . as_secs () . checked_mul (1_000) ; let ms_extra = dur . subsec_millis () ; ms . and_then (| ms | ms . checked_add (ms_extra as u64)) . map (| ms | cmp :: min (u32 :: MAX as u64 , ms) as u32) . unwrap_or (INFINITE - 1) }
};
}
