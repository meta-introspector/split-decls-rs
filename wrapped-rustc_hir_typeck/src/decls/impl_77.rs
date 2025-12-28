macro_rules! deps {
    () => {
        SuggestAnnotations!();
        SuggestAnnotation!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl Subdiagnostic for SuggestAnnotations { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { if self . suggestions . is_empty () { return ; } let mut suggestions = vec ! [] ; for suggestion in self . suggestions { match suggestion { SuggestAnnotation :: Unit (span) => { suggestions . push ((span , "()" . to_string ())) ; } SuggestAnnotation :: Path (span) => { suggestions . push ((span . shrink_to_lo () , "<() as " . to_string ())) ; suggestions . push ((span . shrink_to_hi () , ">" . to_string ())) ; } SuggestAnnotation :: Local (span) => { suggestions . push ((span , ": ()" . to_string ())) ; } SuggestAnnotation :: Turbo (span , n_args , idx) => suggestions . push ((span , format ! ("::<{}>" , (0 .. n_args) . map (| i | if i == idx { "()" } else { "_" }) . collect ::< Vec < _ >> () . join (", ") ,) ,)) , } } diag . multipart_suggestion_verbose ("use `()` annotations to avoid fallback changes" , suggestions , Applicability :: MachineApplicable ,) ; } }
    };
}

impl_77!();