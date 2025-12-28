macro_rules! deps {
    () => {
        AltHead!();
        IfLetRescopeRewrite!();
        SingleArmMatchBegin!();
        ConsequentRewrite!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl Subdiagnostic for IfLetRescopeRewrite { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { let mut suggestions = vec ! [] ; for match_head in self . match_heads { match match_head { SingleArmMatchBegin :: WithOpenBracket (span) => { suggestions . push ((span , "{ match " . into ())) } SingleArmMatchBegin :: WithoutOpenBracket (span) => { suggestions . push ((span , "match " . into ())) } } } for ConsequentRewrite { span , pat } in self . consequent_heads { suggestions . push ((span , format ! ("{{ {pat} => "))) ; } for AltHead (span) in self . alt_heads { suggestions . push ((span , " _ => " . into ())) ; } let closing_brackets = self . closing_brackets ; suggestions . push ((closing_brackets . span , closing_brackets . empty_alt . then_some (" _ => {}" . chars ()) . into_iter () . flatten () . chain (repeat ('}') . take (closing_brackets . count)) . collect () ,)) ; let msg = diag . eagerly_translate (crate :: fluent_generated :: lint_suggestion) ; diag . multipart_suggestion_with_style (msg , suggestions , Applicability :: MachineApplicable , SuggestionStyle :: ShowCode ,) ; } }
    };
}

impl_237!()