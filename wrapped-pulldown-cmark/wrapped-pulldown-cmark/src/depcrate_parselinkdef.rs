// Generated macro for LinkDef (struct)
macro_rules! Depcrate_parseLinkDef {
() => {
// Module: crate::parse
// Provides: {"LinkDef"}
// Dependencies: {}
# [doc = " Contains the destination URL, title and source span of a reference definition."] # [derive (Clone , Debug)] pub struct LinkDef < 'a > { pub dest : CowStr < 'a > , pub title : Option < CowStr < 'a > > , pub span : Range < usize > , }
};
}
