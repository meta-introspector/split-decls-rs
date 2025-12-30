// Generated macro for impl_176 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrowsimpl_176 {
() => {
// Module: crate::borrow_tracker::stacked_borrows
// Provides: {"impl_176"}
// Dependencies: {}
# [doc = " We need to make at least the following things true:"] # [doc = ""] # [doc = " U1: After creating a `Uniq`, it is at the top."] # [doc = " U2: If the top is `Uniq`, accesses must be through that `Uniq` or remove it."] # [doc = " U3: If an access happens with a `Uniq`, it requires the `Uniq` to be in the stack."] # [doc = ""] # [doc = " F1: After creating a `&`, the parts outside `UnsafeCell` have our `SharedReadOnly` on top."] # [doc = " F2: If a write access happens, it pops the `SharedReadOnly`.  This has three pieces:"] # [doc = "     F2a: If a write happens granted by an item below our `SharedReadOnly`, the `SharedReadOnly`"] # [doc = "          gets popped."] # [doc = "     F2b: No `SharedReadWrite` or `Unique` will ever be added on top of our `SharedReadOnly`."] # [doc = " F3: If an access happens with an `&` outside `UnsafeCell`,"] # [doc = "     it requires the `SharedReadOnly` to still be in the stack."] # [doc = ""] # [doc = " Core relation on `Permission` to define which accesses are allowed"] impl Permission { # [doc = " This defines for a given permission, whether it permits the given kind of access."] fn grants (self , access : AccessKind) -> bool { self != Permission :: Disabled && (access == AccessKind :: Read || self != Permission :: SharedReadOnly) } }
};
}
