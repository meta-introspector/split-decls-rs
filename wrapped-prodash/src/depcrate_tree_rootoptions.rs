// Generated macro for Options (struct)
macro_rules! Depcrate_tree_rootOptions {
() => {
// Module: crate::tree::root
// Provides: {"Options"}
// Dependencies: {}
# [doc = " A way to configure new [`tree::Root`](./tree/struct.Root.html) instances"] # [doc = " ```rust"] # [doc = " let tree = prodash::tree::root::Options::default().create();"] # [doc = " let tree2 = prodash::tree::root::Options { message_buffer_capacity: 100, ..Default::default() }.create();"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct Options { # [doc = " The amount of [items][Item] the tree can hold without being forced to allocate."] pub initial_capacity : usize , # [doc = " The amount of messages we can hold before we start overwriting old ones."] pub message_buffer_capacity : usize , }
};
}
