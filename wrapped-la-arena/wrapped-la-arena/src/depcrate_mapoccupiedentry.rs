// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_mapOccupiedEntry {
() => {
// Module: crate::map
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " A view into an occupied entry in a [`ArenaMap`]. It is part of the [`Entry`] enum."] pub struct OccupiedEntry < 'a , IDX , V > { slot : & 'a mut Option < V > , _ty : PhantomData < IDX > , }
};
}
