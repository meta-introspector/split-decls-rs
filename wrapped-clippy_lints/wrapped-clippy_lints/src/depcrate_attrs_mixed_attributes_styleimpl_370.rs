// Generated macro for impl_370 (impl)
macro_rules! Depcrate_attrs_mixed_attributes_styleimpl_370 {
() => {
// Module: crate::attrs::mixed_attributes_style
// Provides: {"impl_370"}
// Dependencies: {}
impl From < & AttrKind > for SimpleAttrKind { fn from (value : & AttrKind) -> Self { match value { AttrKind :: Normal (attr) => { let path_symbols = attr . item . path . segments . iter () . map (| seg | seg . ident . name) . collect :: < Vec < _ > > () ; Self :: Normal (path_symbols) } , AttrKind :: DocComment (..) => Self :: Doc , } } }
};
}
