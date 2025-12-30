// Generated macro for impl_3097 (impl)
macro_rules! Depcrate_scoped_hash_mapimpl_3097 {
() => {
// Module: crate::scoped_hash_map
// Provides: {"impl_3097"}
// Dependencies: {}
impl < 'a , K , V > VacantEntry < 'a , K , V > { # [doc = " Sets the value of the entry with the `VacantEntry`'s key."] pub fn insert (self , value : V) { let val = Val { value , level : self . depth , generation : self . generation , } ; match self . entry { InsertLoc :: Vacant (v) => { v . insert (val) ; } InsertLoc :: Occupied (mut o) => { * o . get_mut () = val ; } } } }
};
}
