// Generated macro for impl_141 (impl)
macro_rules! Depcrate_deimpl_141 {
() => {
// Module: crate::de
// Provides: {"impl_141"}
// Dependencies: {}
impl < 'a > PayloadEvent < 'a > { # [doc = " Ensures that all data is owned to extend the object's lifetime if necessary."] # [inline] fn into_owned (self) -> PayloadEvent < 'static > { match self { PayloadEvent :: Start (e) => PayloadEvent :: Start (e . into_owned ()) , PayloadEvent :: End (e) => PayloadEvent :: End (e . into_owned ()) , PayloadEvent :: Text (e) => PayloadEvent :: Text (e . into_owned ()) , PayloadEvent :: CData (e) => PayloadEvent :: CData (e . into_owned ()) , PayloadEvent :: DocType (e) => PayloadEvent :: DocType (e . into_owned ()) , PayloadEvent :: GeneralRef (e) => PayloadEvent :: GeneralRef (e . into_owned ()) , PayloadEvent :: Eof => PayloadEvent :: Eof , } } }
};
}
