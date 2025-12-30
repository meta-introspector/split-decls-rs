// Generated macro for LocalAnalyzer (struct)
macro_rules! Depcrate_mir_analyzeLocalAnalyzer {
() => {
// Module: crate::mir::analyze
// Provides: {"LocalAnalyzer"}
// Dependencies: {}
struct LocalAnalyzer < 'a , 'b , 'tcx , Bx : BuilderMethods < 'b , 'tcx > > { fx : & 'a FunctionCx < 'b , 'tcx , Bx > , dominators : & 'a Dominators < mir :: BasicBlock > , locals : IndexVec < mir :: Local , LocalKind > , }
};
}
