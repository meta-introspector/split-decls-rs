// Generated macro for cfg (module)
macro_rules! Depcrate_rt_locationcfg {
() => {
// Module: crate::rt::location
// Provides: {"cfg"}
// Dependencies: {}
mod cfg { use std :: fmt ; # [derive (Debug , Default , Clone , Copy)] pub (crate) struct Location (Option < & 'static std :: panic :: Location < 'static > >) ; impl Location { pub (crate) fn from (location : & 'static std :: panic :: Location < 'static >) -> Location { Location (Some (location)) } pub (crate) fn disabled () -> Location { Location (None) } pub (crate) fn is_captured (& self) -> bool { self . 0 . is_some () } } impl fmt :: Display for Location { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (location) = & self . 0 { location . fmt (fmt) } else { write ! (fmt , "") } } } }
};
}
