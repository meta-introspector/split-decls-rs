// Generated macro for impl_293 (impl)
macro_rules! Depcrate_eventsimpl_293 {
() => {
// Module: crate::events
// Provides: {"impl_293"}
// Dependencies: {}
impl < 'a > BytesEnd < 'a > { # [doc = " Internal constructor, used by `Reader`. Supplies data in reader's encoding"] # [inline] pub (crate) const fn wrap (name : Cow < 'a , [u8] >) -> Self { BytesEnd { name } } # [doc = " Creates a new `BytesEnd` borrowing a slice."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " `name` must be a valid name."] # [inline] pub fn new < C : Into < Cow < 'a , str > > > (name : C) -> Self { Self :: wrap (str_cow_to_bytes (name)) } # [doc = " Converts the event into an owned event."] pub fn into_owned (self) -> BytesEnd < 'static > { BytesEnd { name : Cow :: Owned (self . name . into_owned ()) , } } # [doc = " Converts the event into a borrowed event."] # [inline] pub fn borrow (& self) -> BytesEnd < '_ > { BytesEnd { name : Cow :: Borrowed (& self . name) , } } # [doc = " Gets the undecoded raw tag name, as present in the input stream."] # [inline] pub fn name (& self) -> QName < '_ > { QName (& self . name) } # [doc = " Gets the undecoded raw local tag name (excluding namespace) as present"] # [doc = " in the input stream."] # [doc = ""] # [doc = " All content up to and including the first `:` character is removed from the tag name."] # [inline] pub fn local_name (& self) -> LocalName < '_ > { self . name () . into () } }
};
}
