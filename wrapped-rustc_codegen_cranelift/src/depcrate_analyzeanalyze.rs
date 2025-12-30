// Generated macro for analyze (function)
macro_rules! Depcrate_analyzeanalyze {
() => {
// Module: crate::analyze
// Provides: {"analyze"}
// Dependencies: {}
pub (crate) fn analyze (fx : & FunctionCx < '_ , '_ , '_ >) -> IndexVec < Local , SsaKind > { let mut flag_map = fx . mir . local_decls . iter () . map (| _ | SsaKind :: MaybeSsa) . collect :: < IndexVec < Local , SsaKind > > () ; for bb in fx . mir . basic_blocks . iter () { for stmt in bb . statements . iter () { match & stmt . kind { Assign (place_and_rval) => match & place_and_rval . 1 { Rvalue :: Ref (_ , _ , place) | Rvalue :: RawPtr (_ , place) => { flag_map [place . local] = SsaKind :: NotSsa ; } _ => { } } , _ => { } } } } flag_map }
};
}
