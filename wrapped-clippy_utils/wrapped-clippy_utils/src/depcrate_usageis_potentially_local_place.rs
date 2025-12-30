// Generated macro for is_potentially_local_place (function)
macro_rules! Depcrate_usageis_potentially_local_place {
() => {
// Module: crate::usage
// Provides: {"is_potentially_local_place"}
// Dependencies: {}
pub fn is_potentially_local_place (local_id : HirId , place : & Place < '_ >) -> bool { match place . base { PlaceBase :: Local (id) => id == local_id , PlaceBase :: Upvar (_) => { true } , _ => false , } }
};
}
