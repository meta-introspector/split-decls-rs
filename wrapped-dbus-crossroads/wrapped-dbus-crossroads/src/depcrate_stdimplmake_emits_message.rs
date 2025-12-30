// Generated macro for make_emits_message (function)
macro_rules! Depcrate_stdimplmake_emits_message {
() => {
// Module: crate::stdimpl
// Provides: {"make_emits_message"}
// Dependencies: {}
pub (crate) fn make_emits_message < V : dbus :: arg :: Arg + dbus :: arg :: Append > (prop_name : & str , emits_changed : & str , ctx : & Context , v : & V) -> Option < dbus :: Message > { let arr = [prop_name] ; let (d , i) = match emits_changed { "false" => return None , "invalidates" => (None , & arr [..]) , "true" => (Some ((arr [0] , Variant (v))) , & [] [..]) , _ => panic ! ("Invalid value of EmitsChangedSignal: {:?}" , emits_changed) } ; use dbus :: message :: SignalArgs ; use dbus :: blocking :: stdintf :: org_freedesktop_dbus :: PropertiesPropertiesChanged as PPC ; let s : & str = ctx . message () . read1 () . unwrap () ; Some (dbus :: Message :: signal (ctx . path () , & PPC :: INTERFACE . into () , & PPC :: NAME . into ()) . append3 (s , dbus :: arg :: Dict :: new (d) , i)) }
};
}
