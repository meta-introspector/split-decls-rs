// Generated macro for AcceptFn (type)
macro_rules! Depcrate_contextAcceptFn {
() => {
// Module: crate::context
// Provides: {"AcceptFn"}
// Dependencies: {}
type AcceptFn < S > = Box < dyn for < 'sess , 'a > Fn (& mut AcceptContext < '_ , 'sess , S > , & ArgParser < 'a >) + Send + Sync > ;
};
}
