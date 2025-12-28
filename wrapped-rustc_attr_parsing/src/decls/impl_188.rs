macro_rules! deps {
    () => {
        MultipleStabilityLevels!();
        FinalizeContext!();
        AllowedTargets!();
        BodyStabilityParser!();
        AttributeParser!();
        AcceptMapping!();
        Stage!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < S : Stage > AttributeParser < S > for BodyStabilityParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(& [sym :: rustc_default_body_unstable] , template ! (List : & [r#"feature = "name", reason = "...", issue = "N""#]) , | this , cx , args | { reject_outside_std ! (cx) ; if this . stability . is_some () { cx . dcx () . emit_err (session_diagnostics :: MultipleStabilityLevels { span : cx . attr_span }) ; } else if let Some ((feature , level)) = parse_unstability (cx , args) { this . stability = Some ((DefaultBodyStability { level , feature } , cx . attr_span)) ; } } ,)] ; const ALLOWED_TARGETS : AllowedTargets = ALLOWED_TARGETS ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { let (stability , span) = self . stability ? ; Some (AttributeKind :: BodyStability { stability , span }) } }
    };
}

impl_188!();