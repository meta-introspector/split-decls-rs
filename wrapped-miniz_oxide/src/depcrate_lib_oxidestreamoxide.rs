// Generated macro for StreamOxide (struct)
macro_rules! Depcrate_lib_oxideStreamOxide {
() => {
// Module: crate::lib_oxide
// Provides: {"StreamOxide"}
// Dependencies: {}
# [derive (Default)] pub struct StreamOxide < 'io , ST : StateType > { pub next_in : Option < & 'io [u8] > , pub total_in : c_ulong , pub next_out : Option < & 'io mut [u8] > , pub total_out : c_ulong , pub (crate) state : Option < Box < InternalState > > , pub adler : u32 , pub (crate) state_type : std :: marker :: PhantomData < ST > , }
};
}
