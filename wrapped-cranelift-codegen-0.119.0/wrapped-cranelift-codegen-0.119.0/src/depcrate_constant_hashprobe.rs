// Generated macro for probe (function)
macro_rules! Depcrate_constant_hashprobe {
() => {
// Module: crate::constant_hash
// Provides: {"probe"}
// Dependencies: {}
# [doc = " Look for `key` in `table`."] # [doc = ""] # [doc = " The provided `hash` value must have been computed from `key` using the same hash function that"] # [doc = " was used to construct the table."] # [doc = ""] # [doc = " Returns `Ok(idx)` with the table index containing the found entry, or `Err(idx)` with the empty"] # [doc = " sentinel entry if no entry could be found."] pub fn probe < K : Copy + Eq , T : Table < K > + ? Sized > (table : & T , key : K , hash : usize ,) -> Result < usize , usize > { debug_assert ! (table . len () . is_power_of_two ()) ; let mask = table . len () - 1 ; let mut idx = hash ; let mut step = 0 ; loop { idx &= mask ; match table . key (idx) { None => return Err (idx) , Some (k) if k == key => return Ok (idx) , _ => { } } step += 1 ; debug_assert ! (step < table . len ()) ; idx += step ; } }
};
}
