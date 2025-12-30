// Generated macro for PropBuilder (struct)
macro_rules! Depcrate_ifacedescPropBuilder {
() => {
// Module: crate::ifacedesc
// Provides: {"PropBuilder"}
// Dependencies: {}
# [doc = " Struct used to describe a property when building an interface."] # [derive (Debug)] pub struct PropBuilder < 'a , T : 'static , A : 'static > { desc : & 'a mut PropDesc , _dummy : PhantomData < & 'static (T , A) > , emits_changed : EmitsChangedSignal , iface_name : Option < & 'a strings :: Interface < 'static > > , prop_name : String , }
};
}
