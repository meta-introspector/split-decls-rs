// Generated macro for Rc (struct)
macro_rules! Depcrate_rcRc {
() => {
// Module: crate::rc
// Provides: {"Rc"}
// Dependencies: {}
# [doc = " A single-threaded reference-counting pointer. 'Rc' stands for 'Reference"] # [doc = " Counted'."] # [doc = ""] # [doc = " See the [module-level documentation](./index.html) for more details."] # [doc = ""] # [doc = " The inherent methods of `Rc` are all associated functions, which means"] # [doc = " that you have to call them as e.g., [`Rc::get_mut(&mut value)`][get_mut] instead of"] # [doc = " `value.get_mut()`. This avoids conflicts with methods of the inner type `T`."] # [doc = ""] # [doc = " [get_mut]: Rc::get_mut"] # [doc (search_unbox)] # [rustc_diagnostic_item = "Rc"] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_insignificant_dtor] pub struct Rc < T : ? Sized , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { ptr : NonNull < RcInner < T > > , phantom : PhantomData < RcInner < T > > , alloc : A , }
};
}
