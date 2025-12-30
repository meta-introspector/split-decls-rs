// Generated macro for impl_89 (impl)
macro_rules! Depcrateimpl_89 {
() => {
// Module: crate
// Provides: {"impl_89"}
// Dependencies: {}
impl TaskRef < '_ > { # [doc = " Tells if this task will wake up the other task."] # [allow (unreachable_patterns)] fn will_wake (self , other : Self) -> bool { match (self , other) { (Self :: Waker (a) , Self :: Waker (b)) => a . will_wake (b) , # [cfg (all (feature = "std" , not (target_family = "wasm")))] (Self :: Unparker (_) , Self :: Unparker (_)) => { false } _ => false , } } # [doc = " Converts this task reference to a task by cloning."] fn into_task (self) -> Task { match self { Self :: Waker (waker) => Task :: Waker (waker . clone ()) , # [cfg (all (feature = "std" , not (target_family = "wasm")))] Self :: Unparker (unparker) => Task :: Unparker (unparker . clone ()) , } } }
};
}
