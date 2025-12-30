// Generated macro for marker (module)
macro_rules! Depcrate_collections_btree_nodemarker {
() => {
// Module: crate::collections::btree::node
// Provides: {"marker"}
// Dependencies: {}
pub (super) mod marker { use core :: marker :: PhantomData ; pub (crate) enum Leaf { } pub (crate) enum Internal { } pub (crate) enum LeafOrInternal { } pub (crate) enum Owned { } pub (crate) enum Dying { } pub (crate) enum DormantMut { } pub (crate) struct Immut < 'a > (PhantomData < & 'a () >) ; pub (crate) struct Mut < 'a > (PhantomData < & 'a mut () >) ; pub (crate) struct ValMut < 'a > (PhantomData < & 'a mut () >) ; pub (crate) trait BorrowType { # [doc = " If node references of this borrow type allow traversing to other"] # [doc = " nodes in the tree, this constant is set to `true`. It can be used"] # [doc = " for a compile-time assertion."] const TRAVERSAL_PERMIT : bool = true ; } impl BorrowType for Owned { # [doc = " Reject traversal, because it isn't needed. Instead traversal"] # [doc = " happens using the result of `borrow_mut`."] # [doc = " By disabling traversal, and only creating new references to roots,"] # [doc = " we know that every reference of the `Owned` type is to a root node."] const TRAVERSAL_PERMIT : bool = false ; } impl BorrowType for Dying { } impl < 'a > BorrowType for Immut < 'a > { } impl < 'a > BorrowType for Mut < 'a > { } impl < 'a > BorrowType for ValMut < 'a > { } impl BorrowType for DormantMut { } pub (crate) enum KV { } pub (crate) enum Edge { } }
};
}
