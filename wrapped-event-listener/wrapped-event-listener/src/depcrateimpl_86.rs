// Generated macro for impl_86 (impl)
macro_rules! Depcrateimpl_86 {
() => {
// Module: crate
// Provides: {"impl_86"}
// Dependencies: {}
impl Task { fn as_task_ref (& self) -> TaskRef < '_ > { match self { Self :: Waker (waker) => TaskRef :: Waker (waker) , # [cfg (all (feature = "std" , not (target_family = "wasm")))] Self :: Unparker (unparker) => TaskRef :: Unparker (unparker) , } } fn wake (self) { match self { Self :: Waker (waker) => waker . wake () , # [cfg (all (feature = "std" , not (target_family = "wasm")))] Self :: Unparker (unparker) => { unparker . unpark () ; } } } }
};
}
