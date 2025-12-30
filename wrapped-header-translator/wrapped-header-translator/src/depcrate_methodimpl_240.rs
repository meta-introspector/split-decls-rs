// Generated macro for impl_240 (impl)
macro_rules! Depcrate_methodimpl_240 {
() => {
// Module: crate::method
// Provides: {"impl_240"}
// Dependencies: {}
impl MethodArgumentQualifier { pub fn parse (qualifiers : ObjCQualifiers) -> Self { match qualifiers { ObjCQualifiers { in_ : true , inout : false , out : false , bycopy : false , byref : false , oneway : false , } => Self :: In , ObjCQualifiers { in_ : false , inout : true , out : false , bycopy : false , byref : false , oneway : false , } => Self :: Inout , ObjCQualifiers { in_ : false , inout : false , out : true , bycopy : false , byref : false , oneway : false , } => Self :: Out , qualifiers => unreachable ! ("unsupported qualifiers {qualifiers:?}") , } } }
};
}
