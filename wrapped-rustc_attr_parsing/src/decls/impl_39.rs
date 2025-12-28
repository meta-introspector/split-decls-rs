macro_rules! deps {
    () => {
        AttributeParser!();
        FinalizeContext!();
        NakedFunctionIncompatibleAttribute!();
        AcceptMapping!();
        AllowedTargets!();
        Stage!();
        NakedParser!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < S : Stage > AttributeParser < S > for NakedParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(& [sym :: naked] , template ! (Word) , | this , cx , args | { if let Err (span) = args . no_args () { cx . expected_no_args (span) ; return ; } if let Some (earlier) = this . span { let span = cx . attr_span ; cx . warn_unused_duplicate (earlier , span) ; } else { this . span = Some (cx . attr_span) ; } })] ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Warn (Target :: MacroCall) ,]) ; fn finalize (self , cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { const ALLOW_LIST : & [rustc_span :: Symbol] = & [sym :: cfg_trace , sym :: cfg_attr_trace , sym :: test , sym :: ignore , sym :: should_panic , sym :: bench , sym :: allow , sym :: warn , sym :: deny , sym :: forbid , sym :: deprecated , sym :: must_use , sym :: cold , sym :: export_name , sym :: link_section , sym :: linkage , sym :: no_mangle , sym :: instruction_set , sym :: repr , sym :: rustc_std_internal_symbol , sym :: rustc_align , sym :: rustc_align_static , sym :: naked , sym :: doc ,] ; let span = self . span ? ; 'outer : for other_attr in cx . all_attrs { for allowed_attr in ALLOW_LIST { if other_attr . segments () . next () . is_some_and (| i | cx . tools . contains (& i . name)) { continue 'outer ; } if other_attr . word_is (* allowed_attr) { continue 'outer ; } if other_attr . word_is (sym :: target_feature) { if ! cx . features () . naked_functions_target_feature () { feature_err (& cx . sess () , sym :: naked_functions_target_feature , other_attr . span () , "`#[target_feature(/* ... */)]` is currently unstable on `#[naked]` functions" ,) . emit () ; } continue 'outer ; } } cx . emit_err (NakedFunctionIncompatibleAttribute { span : other_attr . span () , naked_span : span , attr : other_attr . get_attribute_path () . to_string () , }) ; } Some (AttributeKind :: Naked (span)) } }
    };
}

impl_39!()