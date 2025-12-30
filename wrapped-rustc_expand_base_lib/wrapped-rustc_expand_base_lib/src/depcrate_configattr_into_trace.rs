// Generated macro for attr_into_trace (function)
macro_rules! Depcrate_configattr_into_trace {
() => {
// Module: crate::config
// Provides: {"attr_into_trace"}
// Dependencies: {}
pub fn attr_into_trace (mut attr : ast :: Attribute , trace_name : Symbol) -> ast :: Attribute { match & mut attr . kind { ast :: AttrKind :: Normal (normal) => { let ast :: NormalAttr { item , tokens } = & mut * * normal ; item . path . segments [0] . ident . name = trace_name ; * tokens = Some (ast :: tokenstream :: LazyAttrTokenStream :: new_direct (ast :: tokenstream :: AttrTokenStream :: default ())) ; } ast :: AttrKind :: DocComment (..) => unreachable ! () , } attr }
};
}
