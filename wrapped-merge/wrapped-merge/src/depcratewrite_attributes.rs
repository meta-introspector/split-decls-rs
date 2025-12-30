// Generated macro for write_attributes (function)
macro_rules! Depcratewrite_attributes {
() => {
// Module: crate
// Provides: {"write_attributes"}
// Dependencies: {}
fn write_attributes < 'a , R : reader :: HasAttributes < 'a > > (writer : & mut writer :: File , parent : writer :: HasAttribute , row : R ,) { for attribute in row . attributes () { let ctor = attribute . ctor () ; let ty = ctor . parent () ; let attribute_ref = writer :: MemberRefParent :: TypeRef (writer . TypeRef (ty . namespace () , ty . name ())) ; let ctor = writer . MemberRef (".ctor" , & ctor . signature (& []) , attribute_ref) ; writer . Attribute (parent , writer :: AttributeType :: MemberRef (ctor) , & attribute . value () ,) ; } }
};
}
