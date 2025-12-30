// Generated macro for impl_27 (impl)
macro_rules! Depcrate_tree_rootimpl_27 {
() => {
// Module: crate::tree::root
// Provides: {"impl_27"}
// Dependencies: {}
impl From < Options > for Root { fn from (Options { initial_capacity , message_buffer_capacity , } : Options ,) -> Self { Root { inner : Mutex :: new (Item { highest_child_id : 0 , value : Arc :: new (AtomicUsize :: default ()) , key : Key :: default () , tree : Arc :: new (crate :: tree :: HashMap :: with_capacity (initial_capacity)) , messages : Arc :: new (Mutex :: new (MessageRingBuffer :: with_capacity (message_buffer_capacity))) , }) , } } }
};
}
