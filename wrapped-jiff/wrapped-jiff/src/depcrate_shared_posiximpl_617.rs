// Generated macro for impl_617 (impl)
macro_rules! Depcrate_shared_posiximpl_617 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_617"}
// Dependencies: {}
impl < 'a , ABBREV > DstInfo < 'a , ABBREV > { # [doc = " Returns true if and only if the given civil datetime ought to be"] # [doc = " considered in DST."] fn in_dst (& self , utc_dt : IDateTime) -> bool { if self . start <= self . end { self . start <= utc_dt && utc_dt < self . end } else { ! (self . end <= utc_dt && utc_dt < self . start) } } # [doc = " Returns the earlier and later times for this DST info."] fn ordered (& self) -> (IDateTime , IDateTime) { if self . start <= self . end { (self . start , self . end) } else { (self . end , self . start) } } # [doc = " Returns the DST offset."] fn offset (& self) -> & PosixOffset { & self . dst . offset } }
};
}
