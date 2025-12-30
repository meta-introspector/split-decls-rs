// Generated macro for skip_uninterested (function)
macro_rules! Depcrate_deskip_uninterested {
() => {
// Module: crate::de
// Provides: {"skip_uninterested"}
// Dependencies: {}
# [doc = " Converts raw reader's event into a payload event."] # [doc = " Returns `None`, if event should be skipped."] # [inline (always)] fn skip_uninterested < 'a > (event : Event < 'a >) -> Option < PayloadEvent < 'a > > { let event = match event { Event :: DocType (e) => PayloadEvent :: DocType (e) , Event :: Start (e) => PayloadEvent :: Start (e) , Event :: End (e) => PayloadEvent :: End (e) , Event :: Eof => PayloadEvent :: Eof , Event :: CData (e) => PayloadEvent :: CData (e) , Event :: Text (e) => PayloadEvent :: Text (e) , Event :: GeneralRef (e) => PayloadEvent :: GeneralRef (e) , _ => return None , } ; Some (event) }
};
}
