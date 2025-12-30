// Generated macro for create_map_1m_integers (function)
macro_rules! Depcratecreate_map_1m_integers {
() => {
// Module: crate
// Provides: {"create_map_1m_integers"}
// Dependencies: {}
fn create_map_1m_integers () -> HashMap < u64 , u64 , FxBuildHasher > { let mut map : HashMap < u64 , u64 , _ > = HashMap :: with_capacity_and_hasher (1_000_000 , FxBuildHasher :: default ()) ; for index in 0 .. map . capacity () { map . insert (index as u64 , index as u64) ; } map }
};
}
