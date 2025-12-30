// Generated macro for get_first_segment (function)
macro_rules! Depcrate_std_instead_of_coreget_first_segment {
() => {
// Module: crate::std_instead_of_core
// Provides: {"get_first_segment"}
// Dependencies: {}
# [doc = " Returns the first named segment of a [`Path`]."] # [doc = ""] # [doc = " If this is a global path (such as `::std::fmt::Debug`), then the segment after [`kw::PathRoot`]"] # [doc = " is returned."] fn get_first_segment < 'tcx > (path : & Path < 'tcx >) -> Option < & 'tcx PathSegment < 'tcx > > { match path . segments { [x , y , ..] if x . ident . name == kw :: PathRoot => Some (y) , [x , ..] => Some (x) , _ => None , } }
};
}
