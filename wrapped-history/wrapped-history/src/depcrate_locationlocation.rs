// Generated macro for Location (struct)
macro_rules! Depcrate_locationLocation {
() => {
// Module: crate::location
// Provides: {"Location"}
// Dependencies: {}
# [doc = " A history location."] # [doc = ""] # [doc = " This struct provides location information at the time"] # [doc = " [`History::location`][crate::History::location] is called."] # [derive (Clone , Debug)] pub struct Location { pub (crate) path : Rc < String > , pub (crate) query_str : Rc < String > , pub (crate) hash : Rc < String > , pub (crate) state : Option < Rc < dyn Any > > , pub (crate) id : Option < u32 > , }
};
}
