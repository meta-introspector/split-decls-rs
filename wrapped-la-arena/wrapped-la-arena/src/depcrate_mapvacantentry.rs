// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_mapVacantEntry {
() => {
// Module: crate::map
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " A view into an vacant entry in a [`ArenaMap`]. It is part of the [`Entry`] enum."] pub struct VacantEntry < 'a , IDX , V > { slot : & 'a mut Option < V > , _ty : PhantomData < IDX > , }
};
}
