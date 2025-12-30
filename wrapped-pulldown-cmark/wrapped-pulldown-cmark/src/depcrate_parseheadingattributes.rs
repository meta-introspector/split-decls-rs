// Generated macro for HeadingAttributes (struct)
macro_rules! Depcrate_parseHeadingAttributes {
() => {
// Module: crate::parse
// Provides: {"HeadingAttributes"}
// Dependencies: {}
# [doc = " Used by the heading attributes extension."] # [derive (Clone)] pub (crate) struct HeadingAttributes < 'a > { pub id : Option < CowStr < 'a > > , pub classes : Vec < CowStr < 'a > > , pub attrs : Vec < (CowStr < 'a > , Option < CowStr < 'a > >) > , }
};
}
