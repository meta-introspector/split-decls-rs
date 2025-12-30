// Generated macro for impl_615 (impl)
macro_rules! Depcrate_seimpl_615 {
() => {
// Module: crate::se
// Provides: {"impl_615"}
// Dependencies: {}
impl < 'i > Indent < 'i > { pub fn borrow (& mut self) -> Indent < '_ > { match self { Self :: None => Indent :: None , Self :: Owned (ref mut i) => Indent :: Borrow (i) , Self :: Borrow (i) => Indent :: Borrow (i) , } } pub fn increase (& mut self) { match self { Self :: None => { } Self :: Owned (i) => i . grow () , Self :: Borrow (i) => i . grow () , } } pub fn decrease (& mut self) { match self { Self :: None => { } Self :: Owned (i) => i . shrink () , Self :: Borrow (i) => i . shrink () , } } pub fn write_indent < W : std :: fmt :: Write > (& mut self , mut writer : W) -> Result < () , SeError > { match self { Self :: None => { } Self :: Owned (i) => { writer . write_char ('\n') ? ; writer . write_str (from_utf8 (i . current ()) ?) ? ; } Self :: Borrow (i) => { writer . write_char ('\n') ? ; writer . write_str (from_utf8 (i . current ()) ?) ? ; } } Ok (()) } }
};
}
