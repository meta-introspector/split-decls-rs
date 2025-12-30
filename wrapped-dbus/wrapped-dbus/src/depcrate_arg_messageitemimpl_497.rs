// Generated macro for impl_497 (impl)
macro_rules! Depcrate_arg_messageitemimpl_497 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_497"}
// Dependencies: {}
impl < 'a > PropHandler < 'a > { # [doc = " Create a new PropHandler from a Props."] pub fn new (p : Props) -> PropHandler { PropHandler { p : p , map : BTreeMap :: new () } } # [doc = " Get a map of all the properties' names and their values."] pub fn get_all (& mut self) -> Result < () , Error > { self . map = self . p . get_all () ? ; Ok (()) } # [doc = " Get a mutable reference to the PropHandler's fetched properties."] pub fn map_mut (& mut self) -> & mut BTreeMap < String , MessageItem > { & mut self . map } # [doc = " Get a reference to the PropHandler's fetched properties."] pub fn map (& self) -> & BTreeMap < String , MessageItem > { & self . map } # [doc = " Get a single property's value."] pub fn get (& mut self , propname : & str) -> Result < & MessageItem , Error > { let v = self . p . get (propname) ? ; self . map . insert (propname . to_string () , v) ; Ok (self . map . get (propname) . unwrap ()) } # [doc = " Set a single property's value."] pub fn set (& mut self , propname : & str , value : MessageItem) -> Result < () , Error > { self . p . set (propname , value . clone ()) ? ; self . map . insert (propname . to_string () , value) ; Ok (()) } }
};
}
