// Generated macro for impl_614 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_614 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_614"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > { # [doc = " Adds a key-value pair to the end of the node, and returns"] # [doc = " a handle to the inserted value."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The returned handle has an unbound lifetime."] pub (super) unsafe fn push_with_handle < 'b > (& mut self , key : K , val : V ,) -> Handle < NodeRef < marker :: Mut < 'b > , K , V , marker :: Leaf > , marker :: KV > { let len = self . len_mut () ; let idx = usize :: from (* len) ; assert ! (idx < CAPACITY) ; * len += 1 ; unsafe { self . key_area_mut (idx) . write (key) ; self . val_area_mut (idx) . write (val) ; Handle :: new_kv (NodeRef { height : self . height , node : self . node , _marker : PhantomData } , idx ,) } } # [doc = " Adds a key-value pair to the end of the node, and returns"] # [doc = " the mutable reference of the inserted value."] pub (super) fn push (& mut self , key : K , val : V) -> * mut V { unsafe { self . push_with_handle (key , val) . into_val_mut () } } }
};
}
