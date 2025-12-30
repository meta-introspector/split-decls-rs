// Generated macro for Node (struct)
macro_rules! DepcrateNode {
() => {
// Module: crate
// Provides: {"Node"}
// Dependencies: {}
# [doc (hidden)] pub struct Node { pub value : & 'static dyn ErasedNode , pub next : UnsafeCell < Option < & 'static Node > > , # [cfg (target_family = "wasm")] pub initialized : AtomicBool , }
};
}
