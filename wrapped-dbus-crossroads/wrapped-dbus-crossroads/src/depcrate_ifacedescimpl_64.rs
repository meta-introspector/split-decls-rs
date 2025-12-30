// Generated macro for impl_64 (impl)
macro_rules! Depcrate_ifacedescimpl_64 {
() => {
// Module: crate::ifacedesc
// Provides: {"impl_64"}
// Dependencies: {}
impl < A : arg :: AppendAll + 'static > SignalBuilder < '_ , A > { # [doc = " Returns a function which, when called, will construct the signal message."] pub fn msg_fn (self) -> Box < dyn Fn (& dbus :: Path , & A) -> dbus :: Message + Send + Sync + 'static > { let SignalBuilder { iface_name , name , .. } = self ; let iface_name : strings :: Interface < 'static > = iface_name . unwrap () . clone () ; Box :: new (move | path , args | { let mut msg = dbus :: Message :: signal (path , & iface_name , & name) ; let mut ia = arg :: IterAppend :: new (& mut msg) ; args . append (& mut ia) ; msg }) } }
};
}
