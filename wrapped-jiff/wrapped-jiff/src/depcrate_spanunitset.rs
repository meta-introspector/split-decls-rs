// Generated macro for UnitSet (struct)
macro_rules! Depcrate_spanUnitSet {
() => {
// Module: crate::span
// Provides: {"UnitSet"}
// Dependencies: {}
# [doc = " A bit set that keeps track of all non-zero units on a `Span`."] # [doc = ""] # [doc = " Because of alignment, adding this to a `Span` does not make it any bigger."] # [doc = ""] # [doc = " The benefit of this bit set is to make it extremely cheap to enable fast"] # [doc = " paths in various places. For example, doing arithmetic on a `Date` with an"] # [doc = " arbitrary `Span` is pretty involved. But if you know the `Span` only"] # [doc = " consists of non-zero units of days (and zero for all other units), then you"] # [doc = " can take a much cheaper path."] # [derive (Clone , Copy)] pub (crate) struct UnitSet (u16) ;
};
}
