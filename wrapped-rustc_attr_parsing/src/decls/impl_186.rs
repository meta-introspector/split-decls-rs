macro_rules! deps {
    () => {
        UnstableFeatureBoundIncompatibleStability!();
        RustcAllowedUnstablePairing!();
        AttributeParser!();
        StabilityParser!();
        FinalizeContext!();
        Stage!();
        AcceptMapping!();
        AllowedTargets!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < S : Stage > AttributeParser < S > for StabilityParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(& [sym :: stable] , template ! (List : & [r#"feature = "name", since = "version""#]) , | this , cx , args | { reject_outside_std ! (cx) ; if ! this . check_duplicate (cx) && let Some ((feature , level)) = parse_stability (cx , args) { this . stability = Some ((Stability { level , feature } , cx . attr_span)) ; } } ,) , (& [sym :: unstable] , template ! (List : & [r#"feature = "name", reason = "...", issue = "N""#]) , | this , cx , args | { reject_outside_std ! (cx) ; if ! this . check_duplicate (cx) && let Some ((feature , level)) = parse_unstability (cx , args) { this . stability = Some ((Stability { level , feature } , cx . attr_span)) ; } } ,) , (& [sym :: rustc_allowed_through_unstable_modules] , template ! (NameValueStr : "deprecation message") , | this , cx , args | { reject_outside_std ! (cx) ; let Some (nv) = args . name_value () else { cx . expected_name_value (cx . attr_span , None) ; return ; } ; let Some (value_str) = nv . value_as_str () else { cx . expected_string_literal (nv . value_span , Some (nv . value_as_lit ())) ; return ; } ; this . allowed_through_unstable_modules = Some (value_str) ; } ,) ,] ; const ALLOWED_TARGETS : AllowedTargets = ALLOWED_TARGETS ; fn finalize (mut self , cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { if let Some (atum) = self . allowed_through_unstable_modules { if let Some ((Stability { level : StabilityLevel :: Stable { ref mut allowed_through_unstable_modules , .. } , .. } , _ ,)) = self . stability { * allowed_through_unstable_modules = Some (atum) ; } else { cx . dcx () . emit_err (session_diagnostics :: RustcAllowedUnstablePairing { span : cx . target_span , }) ; } } if let Some ((Stability { level : StabilityLevel :: Stable { .. } , .. } , _)) = self . stability { for other_attr in cx . all_attrs { if other_attr . word_is (sym :: unstable_feature_bound) { cx . emit_err (session_diagnostics :: UnstableFeatureBoundIncompatibleStability { span : cx . target_span , }) ; } } } let (stability , span) = self . stability ? ; Some (AttributeKind :: Stability { stability , span }) } }
    };
}

impl_186!();