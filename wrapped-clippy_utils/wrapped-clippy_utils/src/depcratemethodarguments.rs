// Generated macro for MethodArguments (type)
macro_rules! DepcrateMethodArguments {
() => {
// Module: crate
// Provides: {"MethodArguments"}
// Dependencies: {}
# [doc = " Arguments of a method: the receiver and all the additional arguments."] pub type MethodArguments < 'tcx > = Vec < (& 'tcx Expr < 'tcx > , & 'tcx [Expr < 'tcx >]) > ;
};
}
