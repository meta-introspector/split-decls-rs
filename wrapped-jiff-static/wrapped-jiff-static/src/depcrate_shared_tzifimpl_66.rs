// Generated macro for impl_66 (impl)
macro_rules! Depcrate_shared_tzifimpl_66 {
() => {
// Module: crate::shared::tzif
// Provides: {"impl_66"}
// Dependencies: {}
impl TzifTransitionsOwned { # [doc = " Add a single transition with the given timestamp."] # [doc = ""] # [doc = " This also fills in the other columns (civil starts, civil ends and"] # [doc = " infos) with sensible default values. It is expected that callers will"] # [doc = " later fill them in."] fn add (& mut self , timestamp : i64) { self . add_with_type_index (timestamp , 0) ; } # [doc = " Like `TzifTransitionsOwned::add`, but let's the caller provide a type"] # [doc = " index if it is known."] fn add_with_type_index (& mut self , timestamp : i64 , type_index : u8) { self . timestamps . push (timestamp) ; self . civil_starts . push (TzifDateTime :: ZERO) ; self . civil_ends . push (TzifDateTime :: ZERO) ; self . infos . push (TzifTransitionInfo { type_index , kind : TzifTransitionKind :: Unambiguous , }) ; } }
};
}
