// Generated macro for impl_1265 (impl)
macro_rules! Depcrate_rcimpl_1265 {
() => {
// Module: crate::rc
// Provides: {"impl_1265"}
// Dependencies: {}
# [stable (feature = "shared_from_cow" , since = "1.45.0")] impl < 'a , B > From < Cow < 'a , B > > for Rc < B > where B : ToOwned + ? Sized , Rc < B > : From < & 'a B > + From < B :: Owned > , { # [doc = " Creates a reference-counted pointer from a clone-on-write pointer by"] # [doc = " copying its content."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use std::rc::Rc;"] # [doc = " # use std::borrow::Cow;"] # [doc = " let cow: Cow<'_, str> = Cow::Borrowed(\"eggplant\");"] # [doc = " let shared: Rc<str> = Rc::from(cow);"] # [doc = " assert_eq!(\"eggplant\", &shared[..]);"] # [doc = " ```"] # [inline] fn from (cow : Cow < 'a , B >) -> Rc < B > { match cow { Cow :: Borrowed (s) => Rc :: from (s) , Cow :: Owned (s) => Rc :: from (s) , } } }
};
}
