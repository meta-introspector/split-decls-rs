// Generated macro for DormantMutRef (struct)
macro_rules! Depcrate_collections_btree_borrowDormantMutRef {
() => {
// Module: crate::collections::btree::borrow
// Provides: {"DormantMutRef"}
// Dependencies: {}
# [doc = " Models a reborrow of some unique reference, when you know that the reborrow"] # [doc = " and all its descendants (i.e., all pointers and references derived from it)"] # [doc = " will not be used any more at some point, after which you want to use the"] # [doc = " original unique reference again."] # [doc = ""] # [doc = " The borrow checker usually handles this stacking of borrows for you, but"] # [doc = " some control flows that accomplish this stacking are too complicated for"] # [doc = " the compiler to follow. A `DormantMutRef` allows you to check borrowing"] # [doc = " yourself, while still expressing its stacked nature, and encapsulating"] # [doc = " the raw pointer code needed to do this without undefined behavior."] pub (super) struct DormantMutRef < 'a , T > { ptr : NonNull < T > , _marker : PhantomData < & 'a mut T > , }
};
}
