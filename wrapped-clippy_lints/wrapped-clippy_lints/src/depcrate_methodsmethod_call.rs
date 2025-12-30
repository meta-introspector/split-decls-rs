// Generated macro for method_call (function)
macro_rules! Depcrate_methodsmethod_call {
() => {
// Module: crate::methods
// Provides: {"method_call"}
// Dependencies: {}
# [doc = " Extracts a method call name, args, and `Span` of the method name."] # [doc = " This ensures that neither the receiver nor any of the arguments"] # [doc = " come from expansion."] pub fn method_call < 'tcx > (recv : & 'tcx Expr < 'tcx >) -> Option < (Symbol , & 'tcx Expr < 'tcx > , & 'tcx [Expr < 'tcx >] , Span , Span) > { if let ExprKind :: MethodCall (path , receiver , args , call_span) = recv . kind && ! args . iter () . any (| e | e . range_span () . unwrap_or (e . span) . from_expansion ()) && ! receiver . range_span () . unwrap_or (receiver . span) . from_expansion () { Some ((path . ident . name , receiver , args , path . ident . span , call_span)) } else { None } }
};
}
