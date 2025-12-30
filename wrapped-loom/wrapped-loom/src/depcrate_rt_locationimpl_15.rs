// Generated macro for impl_15 (impl)
macro_rules! Depcrate_rt_locationimpl_15 {
() => {
// Module: crate::rt::location
// Provides: {"impl_15"}
// Dependencies: {}
impl PanicBuilder { pub (super) fn location (& mut self , key : & str , location : Location) -> & mut Self { self . locations . push ((key . to_string () , None , location)) ; self } pub (super) fn thread (& mut self , key : & str , thread : impl Into < usize > , location : Location ,) -> & mut Self { self . locations . push ((key . to_string () , Some (thread . into ()) , location)) ; self } pub (super) fn fire (& self) { let mut msg = self . msg . clone () ; let width = self . locations . iter () . filter (| (_ , _ , location) | location . is_captured ()) . map (| (key , ..) | key . len ()) . max () ; if let Some (width) = width { msg = format ! ("\n{}" , msg) ; for (key , thread , location) in & self . locations { if ! location . is_captured () { continue ; } let spaces : String = (0 .. width - key . len ()) . map (| _ | " ") . collect () ; let th = thread . map (| th | format ! ("thread #{} @ " , th)) . unwrap_or_else (String :: new) ; msg . push_str (& format ! ("\n    {}{}: {}{}" , spaces , key , th , location)) ; } } panic ! ("{}\n" , msg) ; } }
};
}
