// Generated macro for HeapVisitor (struct)
macro_rules! Depcrate_hir_visitorHeapVisitor {
() => {
// Module: crate::hir::visitor
// Provides: {"HeapVisitor"}
// Dependencies: {}
# [doc = " HeapVisitor visits every item in an `Hir` recursively using constant stack"] # [doc = " size and a heap size proportional to the size of the `Hir`."] struct HeapVisitor < 'a > { # [doc = " A stack of `Hir` nodes. This is roughly analogous to the call stack"] # [doc = " used in a typical recursive visitor."] stack : Vec < (& 'a Hir , Frame < 'a >) > , }
};
}
