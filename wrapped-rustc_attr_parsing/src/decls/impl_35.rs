macro_rules! deps {
    () => {
        SingleAttributeParser!();
        Stage!();
        OnDuplicate!();
        AllowedTargets!();
        AcceptContext!();
        CoverageParser!();
        ArgParser!();
        AttributeOrder!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for CoverageParser { const PATH : & [Symbol] = & [sym :: coverage] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Closure) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Impl { of_trait : true }) , Allow (Target :: Impl { of_trait : false }) , Allow (Target :: Mod) , Allow (Target :: Crate) ,]) ; const TEMPLATE : AttributeTemplate = template ! (OneOf : & [sym :: off , sym :: on]) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let Some (args) = args . list () else { cx . expected_specific_argument_and_list (cx . attr_span , & [sym :: on , sym :: off]) ; return None ; } ; let Some (arg) = args . single () else { cx . expected_single_argument (args . span) ; return None ; } ; let fail_incorrect_argument = | span | cx . expected_specific_argument (span , & [sym :: on , sym :: off]) ; let Some (arg) = arg . meta_item () else { fail_incorrect_argument (args . span) ; return None ; } ; let kind = match arg . path () . word_sym () { Some (sym :: off) => CoverageAttrKind :: Off , Some (sym :: on) => CoverageAttrKind :: On , None | Some (_) => { fail_incorrect_argument (arg . span ()) ; return None ; } } ; Some (AttributeKind :: Coverage (cx . attr_span , kind)) } }
    };
}

impl_35!();