// Generated macro for impl_86 (impl)
macro_rules! Depcrate_memoryimpl_86 {
() => {
// Module: crate::memory
// Provides: {"impl_86"}
// Dependencies: {}
impl Default for LocationStack { fn default () -> Self { Self { prev : Vec :: new () , next : VecDeque :: new () , current : Location { path : "/" . to_string () . into () , query_str : "" . to_string () . into () , hash : "" . to_string () . into () , state : None , id : Some (get_id ()) , } , } } }
};
}
