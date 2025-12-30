// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl < T > ChunkList < T > { # [inline (never)] # [cold] fn reserve (& mut self , additional : usize) { let double_cap = self . current . capacity () . checked_mul (2) . expect ("capacity overflow") ; let required_cap = additional . checked_next_power_of_two () . expect ("capacity overflow") ; let new_capacity = cmp :: max (double_cap , required_cap) ; let chunk = mem :: replace (& mut self . current , Vec :: with_capacity (new_capacity)) ; self . rest . push (chunk) ; } }
};
}
