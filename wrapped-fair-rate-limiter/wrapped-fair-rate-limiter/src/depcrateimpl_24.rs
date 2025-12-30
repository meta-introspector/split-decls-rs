// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl RecentCosts { # [must_use] pub fn new (now : Instant) -> Self { Self { cost : 0_u32 , last : now , } } pub fn is_empty (& self) -> bool { self . cost == 0 } pub fn add (& mut self , cost : u32) { self . cost . saturating_add_assign (cost) ; } pub fn update (& mut self , tick_duration : Duration , now : Instant) { let elapsed = now . saturating_duration_since (self . last) ; # [allow (clippy :: cast_possible_truncation)] let elapsed_ticks = (elapsed . as_millis () / tick_duration . as_millis ()) as u32 ; self . last += tick_duration * elapsed_ticks ; self . cost = self . cost . wrapping_shr (elapsed_ticks) ; } # [must_use] pub fn recent_cost (& self) -> u32 { self . cost } }
};
}
