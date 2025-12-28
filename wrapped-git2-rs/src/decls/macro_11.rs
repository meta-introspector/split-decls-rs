macro_rules! macro_11 {
    () => {
        bitflags ! { # [doc = " Orderings that may be specified for Revwalk iteration."] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct Sort : u32 { # [doc = " Sort the repository contents in no particular ordering."] # [doc = ""] # [doc = " This sorting is arbitrary, implementation-specific, and subject to"] # [doc = " change at any time. This is the default sorting for new walkers."] const NONE = raw :: GIT_SORT_NONE as u32 ; # [doc = " Sort the repository contents in topological order (children before"] # [doc = " parents)."] # [doc = ""] # [doc = " This sorting mode can be combined with time sorting."] const TOPOLOGICAL = raw :: GIT_SORT_TOPOLOGICAL as u32 ; # [doc = " Sort the repository contents by commit time."] # [doc = ""] # [doc = " This sorting mode can be combined with topological sorting."] const TIME = raw :: GIT_SORT_TIME as u32 ; # [doc = " Iterate through the repository contents in reverse order."] # [doc = ""] # [doc = " This sorting mode can be combined with any others."] const REVERSE = raw :: GIT_SORT_REVERSE as u32 ; } }
    };
}

macro_11!()