// Generated macro for IfaceDesc (struct)
macro_rules! Depcrate_ifacedescIfaceDesc {
() => {
// Module: crate::ifacedesc
// Provides: {"IfaceDesc"}
// Dependencies: {}
# [derive (Debug)] pub struct IfaceDesc { name : Option < strings :: Interface < 'static > > , annotations : Annotations , methods : HashMap < strings :: Member < 'static > , MethodDesc > , signals : HashMap < strings :: Member < 'static > , SignalDesc > , properties : HashMap < String , PropDesc > , }
};
}
