// Generated macro for Stack (struct)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_stackStack {
() => {
// Module: crate::borrow_tracker::stacked_borrows::stack
// Provides: {"Stack"}
// Dependencies: {}
# [doc = " Extra per-location state."] # [derive (Clone , Debug)] pub struct Stack { # [doc = " Used *mostly* as a stack; never empty."] # [doc = " Invariants:"] # [doc = " * Above a `SharedReadOnly` there can only be more `SharedReadOnly`."] # [doc = " * Except for `Untagged`, no tag occurs in the stack more than once."] borrows : Vec < Item > , # [doc = " If this is `Some(id)`, then the actual current stack is unknown. This can happen when"] # [doc = " wildcard pointers are used to access this location. What we do know is that `borrows` are at"] # [doc = " the top of the stack, and below it are arbitrarily many items whose `tag` is strictly less"] # [doc = " than `id`."] # [doc = " When the bottom is unknown, `borrows` always has a `SharedReadOnly` or `Unique` at the bottom;"] # [doc = " we never have the unknown-to-known boundary in an SRW group."] unknown_bottom : Option < BorTag > , # [doc = " A small LRU cache of searches of the borrow stack."] # [cfg (feature = "stack-cache")] cache : StackCache , # [doc = " On a read, we need to disable all `Unique` above the granting item. We can avoid most of"] # [doc = " this scan by keeping track of the region of the borrow stack that may contain `Unique`s."] # [cfg (feature = "stack-cache")] unique_range : Range < usize > , }
};
}
