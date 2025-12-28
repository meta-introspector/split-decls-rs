macro_rules! deps {
    () => {
        Candidate!();
        IsSuggestion!();
        CandidateSource!();
        Mode!();
        FnCtxt!();
    };
}

macro_rules! ProbeContext {
    () => {
        deps!();
        pub (crate) struct ProbeContext < 'a , 'tcx > { fcx : & 'a FnCtxt < 'a , 'tcx > , span : Span , mode : Mode , method_name : Option < Ident > , return_type : Option < Ty < 'tcx > > , # [doc = " This is the OriginalQueryValues for the steps queries"] # [doc = " that are answered in steps."] orig_steps_var_values : & 'a OriginalQueryValues < 'tcx > , steps : & 'tcx [CandidateStep < 'tcx >] , inherent_candidates : Vec < Candidate < 'tcx > > , extension_candidates : Vec < Candidate < 'tcx > > , impl_dups : FxHashSet < DefId > , # [doc = " When probing for names, include names that are close to the"] # [doc = " requested name (by edit distance)"] allow_similar_names : bool , # [doc = " List of potential private candidates. Will be trimmed to ones that"] # [doc = " actually apply and then the result inserted into `private_candidate`"] private_candidates : Vec < Candidate < 'tcx > > , # [doc = " Some(candidate) if there is a private candidate"] private_candidate : Cell < Option < (DefKind , DefId) > > , # [doc = " Collects near misses when the candidate functions are missing a `self` keyword and is only"] # [doc = " used for error reporting"] static_candidates : RefCell < Vec < CandidateSource > > , scope_expr_id : HirId , # [doc = " Is this probe being done for a diagnostic? This will skip some error reporting"] # [doc = " machinery, since we don't particularly care about, for example, similarly named"] # [doc = " candidates if we're *reporting* similarly named candidates."] is_suggestion : IsSuggestion , }
    };
}

ProbeContext!()