// Generated macro for ConstDebugInfo (struct)
macro_rules! Depcrate_mir_debuginfoConstDebugInfo {
() => {
// Module: crate::mir::debuginfo
// Provides: {"ConstDebugInfo"}
// Dependencies: {}
# [doc = " Information needed to emit a constant."] pub struct ConstDebugInfo < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > { pub name : String , pub source_info : mir :: SourceInfo , pub operand : OperandRef < 'tcx , Bx :: Value > , pub dbg_var : Bx :: DIVariable , pub dbg_loc : Bx :: DILocation , pub fragment : Option < Range < Size > > , pub _phantom : PhantomData < & 'a () > , }
};
}
