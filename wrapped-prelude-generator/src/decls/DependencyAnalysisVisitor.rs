macro_rules! DependencyAnalysisVisitor {
    () => {
        # [derive (Debug , Default)] pub struct DependencyAnalysisVisitor { pub dependencies : HashSet < String > , pub types_used : HashSet < String > , }
    };
}

DependencyAnalysisVisitor!()