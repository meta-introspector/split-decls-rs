// Generated macro for impl_371 (impl)
macro_rules! Depcrate_inlay_hintsimpl_371 {
() => {
// Module: crate::inlay_hints
// Provides: {"impl_371"}
// Dependencies: {}
impl InlayHintLabel { pub fn simple (s : impl Into < String > , tooltip : Option < LazyProperty < InlayTooltip > > , linked_location : Option < LazyProperty < FileRange > > ,) -> InlayHintLabel { InlayHintLabel { parts : smallvec ! [InlayHintLabelPart { text : s . into () , linked_location , tooltip }] , } } pub fn prepend_str (& mut self , s : & str) { match & mut * self . parts { [InlayHintLabelPart { text , linked_location : None , tooltip : None } , ..] => { text . insert_str (0 , s) } _ => self . parts . insert (0 , InlayHintLabelPart { text : s . into () , linked_location : None , tooltip : None } ,) , } } pub fn append_str (& mut self , s : & str) { match & mut * self . parts { [.. , InlayHintLabelPart { text , linked_location : None , tooltip : None }] => { text . push_str (s) } _ => self . parts . push (InlayHintLabelPart { text : s . into () , linked_location : None , tooltip : None , }) , } } pub fn append_part (& mut self , part : InlayHintLabelPart) { if part . linked_location . is_none () && part . tooltip . is_none () && let Some (InlayHintLabelPart { text , linked_location : None , tooltip : None }) = self . parts . last_mut () { text . push_str (& part . text) ; return ; } self . parts . push (part) ; } }
};
}
