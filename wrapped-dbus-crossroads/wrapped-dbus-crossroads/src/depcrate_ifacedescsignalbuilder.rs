// Generated macro for SignalBuilder (struct)
macro_rules! Depcrate_ifacedescSignalBuilder {
() => {
// Module: crate::ifacedesc
// Provides: {"SignalBuilder"}
// Dependencies: {}
# [doc = " Struct used to describe a property when building an interface."] # [derive (Debug)] pub struct SignalBuilder < 'a , A : 'static > { desc : & 'a mut SignalDesc , _dummy : PhantomData < & 'static A > , iface_name : Option < & 'a strings :: Interface < 'static > > , name : strings :: Member < 'static > , }
};
}
