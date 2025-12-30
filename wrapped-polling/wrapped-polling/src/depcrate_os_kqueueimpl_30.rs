// Generated macro for impl_30 (impl)
macro_rules! Depcrate_os_kqueueimpl_30 {
() => {
// Module: crate::os::kqueue
// Provides: {"impl_30"}
// Dependencies: {}
impl Events { # [doc = " Creates an empty list."] pub fn with_capacity (cap : usize) -> Events { Events { list : Vec :: with_capacity (cap) , } } # [doc = " Iterates over I/O events."] pub fn iter (& self) -> impl Iterator < Item = Event > + '_ { self . list . iter () . map (| ev | Event { key : ev . udata () as usize , readable : matches ! (ev . filter () , kqueue :: EventFilter :: Read (..) | kqueue :: EventFilter :: Vnode { .. } | kqueue :: EventFilter :: Proc { .. } | kqueue :: EventFilter :: Signal { .. } | kqueue :: EventFilter :: Timer { .. }) , writable : matches ! (ev . filter () , kqueue :: EventFilter :: Write (..)) || (matches ! (ev . filter () , kqueue :: EventFilter :: Read (..)) && (ev . flags () . intersects (kqueue :: EventFlags :: EOF))) , extra : EventExtra , }) } # [doc = " Clears the list."] pub fn clear (& mut self) { self . list . clear () ; } # [doc = " Get the capacity of the list."] pub fn capacity (& self) -> usize { self . list . capacity () } }
};
}
