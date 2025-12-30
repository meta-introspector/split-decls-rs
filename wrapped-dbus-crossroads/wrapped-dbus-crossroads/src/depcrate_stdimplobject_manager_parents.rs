// Generated macro for object_manager_parents (function)
macro_rules! Depcrate_stdimplobject_manager_parents {
() => {
// Module: crate::stdimpl
// Provides: {"object_manager_parents"}
// Dependencies: {}
fn object_manager_parents < F : FnMut (dbus :: Path < 'static > , & mut Crossroads) > (name : & dbus :: Path < 'static > , cr : & mut Crossroads , mut f : F) { for idx in 0 .. name . len () - 1 { if name . as_bytes () [idx] != b'/' { continue ; } let parent = dbus :: Path :: from (& name [0 .. (if idx == 0 { idx + 1 } else { idx })]) . into_static () ; if ! cr . has_interface (& parent , cr . object_manager :: < () > ()) { continue ; } f (parent , cr) } }
};
}
