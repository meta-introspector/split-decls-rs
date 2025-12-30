// Generated macro for impl_501 (impl)
macro_rules! Depcrate_readerimpl_501 {
() => {
// Module: crate::reader
// Provides: {"impl_501"}
// Dependencies: {}
# [doc = " Private sync reading methods"] impl < R > Reader < R > { # [doc = " Read text into the given buffer, and return an event that borrows from"] # [doc = " either that buffer or from the input itself, based on the type of the"] # [doc = " reader."] fn read_event_impl < 'i , B > (& mut self , mut buf : B) -> Result < Event < 'i > , Error > where R : XmlSource < 'i , B > , { read_event_impl ! (self , buf , self . reader , read_until_close) } # [doc = " Private function to read until `>` is found. This function expects that"] # [doc = " it was called just after encounter a `<` symbol."] fn read_until_close < 'i , B > (& mut self , buf : B) -> Result < Event < 'i > , Error > where R : XmlSource < 'i , B > , { read_until_close ! (self , buf , self . reader) } }
};
}
