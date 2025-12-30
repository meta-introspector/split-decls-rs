// Generated macro for impl_13 (impl)
macro_rules! Depcrate_eventimpl_13 {
() => {
// Module: crate::event
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a > Event < 'a > { # [doc = " Returns true if the time interval of `self` completely contains the"] # [doc = " time interval of `other`."] pub fn contains (& self , other : & Event < '_ >) -> bool { self . payload . contains (& other . payload) } pub fn duration (& self) -> Option < Duration > { self . payload . duration () } pub fn integer (& self) -> Option < u64 > { self . payload . integer () } pub (crate) fn parse_event_id (event_id : Cow < 'a , str >) -> (Cow < 'a , str > , Vec < Cow < 'a , str > >) { let event_id = match event_id { Cow :: Owned (s) => Cow :: Owned (s . into_bytes ()) , Cow :: Borrowed (s) => Cow :: Borrowed (s . as_bytes ()) , } ; let mut parser = Parser :: new (event_id) ; let label = match parser . parse_label () { Ok (label) => label , Err (message) => { eprintln ! ("{}" , message) ; return (Cow :: from ("<parse error>") , Vec :: new ()) ; } } ; let mut args = Vec :: new () ; while parser . pos != parser . full_text . len () { match parser . parse_arg () { Ok (arg) => args . push (arg) , Err (message) => { eprintln ! ("{}" , message) ; break ; } } } (label , args) } }
};
}
