// Generated macro for STATIC_INNER_SLICE (static)
macro_rules! Depcrate_syncSTATIC_INNER_SLICE {
() => {
// Module: crate::sync
// Provides: {"STATIC_INNER_SLICE"}
// Dependencies: {}
static STATIC_INNER_SLICE : SliceArcInnerForStatic = SliceArcInnerForStatic { inner : ArcInner { strong : atomic :: AtomicUsize :: new (1) , weak : atomic :: AtomicUsize :: new (1) , data : [0] , } , } ;
};
}
