macro_rules! deps {
    () => {
        Error!();
        CandidateSource!();
        NoMatchData!();
    };
}

macro_rules! MethodError {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) enum MethodError < 'tcx > { # [doc = " Did not find an applicable method, but we did find various near-misses that may work."] NoMatch (NoMatchData < 'tcx >) , # [doc = " Multiple methods might apply."] Ambiguity (Vec < CandidateSource >) , # [doc = " Found an applicable method, but it is not visible. The third argument contains a list of"] # [doc = " not-in-scope traits which may work."] PrivateMatch (DefKind , DefId , Vec < DefId >) , # [doc = " Found a `Self: Sized` bound where `Self` is a trait object."] IllegalSizedBound { candidates : Vec < DefId > , needs_mut : bool , bound_span : Span , self_expr : & 'tcx hir :: Expr < 'tcx > , } , # [doc = " Found a match, but the return type is wrong"] BadReturnType , # [doc = " Error has already been emitted, no need to emit another one."] ErrorReported (ErrorGuaranteed) , }
    };
}

MethodError!()