macro_rules! OptimizationDiagnosticKind {
    () => {
        # [derive (Copy , Clone , Debug)] pub (crate) enum OptimizationDiagnosticKind { OptimizationRemark , OptimizationMissed , OptimizationAnalysis , OptimizationAnalysisFPCommute , OptimizationAnalysisAliasing , OptimizationFailure , OptimizationRemarkOther , }
    };
}

OptimizationDiagnosticKind!()