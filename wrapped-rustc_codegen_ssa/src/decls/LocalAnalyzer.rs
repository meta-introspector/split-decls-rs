macro_rules! deps {
    () => {
        FunctionCx!();
        LocalKind!();
        BuilderMethods!();
    };
}

macro_rules! LocalAnalyzer {
    () => {
        deps!();
        struct LocalAnalyzer < 'a , 'b , 'tcx , Bx : BuilderMethods < 'b , 'tcx > > { fx : & 'a FunctionCx < 'b , 'tcx , Bx > , dominators : & 'a Dominators < mir :: BasicBlock > , locals : IndexVec < mir :: Local , LocalKind > , }
    };
}

LocalAnalyzer!()