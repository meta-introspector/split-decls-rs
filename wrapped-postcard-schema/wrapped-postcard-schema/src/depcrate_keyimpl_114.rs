// Generated macro for impl_114 (impl)
macro_rules! Depcrate_keyimpl_114 {
() => {
// Module: crate::key
// Provides: {"impl_114"}
// Dependencies: {}
impl core :: fmt :: Debug for Key { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str ("Key(") ? ; for b in self . 0 . iter () { f . write_fmt (format_args ! ("{b} ")) ? ; } f . write_str (")") } }
};
}
