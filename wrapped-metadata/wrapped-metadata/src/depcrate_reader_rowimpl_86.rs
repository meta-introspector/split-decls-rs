// Generated macro for impl_86 (impl)
macro_rules! Depcrate_reader_rowimpl_86 {
() => {
// Module: crate::reader::row
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'a , R : AsRow < 'a > + Into < HasAttribute < 'a > > > HasAttributes < 'a > for R { fn attributes (& self) -> RowIterator < 'a , Attribute < 'a > > { self . equal_range (0 , Into :: < HasAttribute > :: into (* self) . encode ()) } fn find_attribute (& self , name : & str) -> Option < Attribute < 'a > > { self . attributes () . find (| attribute | attribute . ctor () . parent () . name () == name) } fn has_attribute (& self , name : & str) -> bool { self . find_attribute (name) . is_some () } }
};
}
