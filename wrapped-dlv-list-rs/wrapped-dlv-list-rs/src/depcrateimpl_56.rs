// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl < T > Entry < T > { # [doc = " Returns the occupied entry by moving it out of the entry."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the variant is actually [`Entry::Vacant`]."] # [must_use] pub fn occupied (self) -> OccupiedEntry < T > { match self { Entry :: Occupied (entry) => entry , Entry :: Vacant (_) => panic ! ("expected occupied entry") , } } # [doc = " Returns an immutable reference to the occupied entry."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the variant is actually [`Entry::Vacant`]."] # [must_use] pub fn occupied_ref (& self) -> & OccupiedEntry < T > { match self { Entry :: Occupied (entry) => entry , Entry :: Vacant (_) => panic ! ("expected occupied entry") , } } # [doc = " Returns a mutable reference to the occupied entry."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the variant is actually [`Entry::Vacant`]."] # [must_use] pub fn occupied_mut (& mut self) -> & mut OccupiedEntry < T > { match self { Entry :: Occupied (entry) => entry , Entry :: Vacant (_) => panic ! ("expected occupied entry") , } } # [doc = " Returns an immutable reference to the vacant entry."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the variant is actually [`Entry::Occupied`]."] # [must_use] pub fn vacant_ref (& self) -> & VacantEntry { match self { Entry :: Vacant (entry) => entry , Entry :: Occupied (_) => panic ! ("expected vacant entry") , } } }
};
}
