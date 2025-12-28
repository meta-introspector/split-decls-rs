macro_rules! deps {
    () => {
        AttributeOrder!();
        ArgParser!();
        OnDuplicate!();
        AcceptContext!();
        RustcBuiltinMacroParser!();
        Stage!();
        AllowedTargets!();
        SingleAttributeParser!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for RustcBuiltinMacroParser { const PATH : & [Symbol] = & [sym :: rustc_builtin_macro] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: MacroDef)]) ; const TEMPLATE : AttributeTemplate = template ! (List : & ["TraitName" , "TraitName, attributes(name1, name2, ...)"]) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let (builtin_name , helper_attrs) = parse_derive_like (cx , args , false) ? ; Some (AttributeKind :: RustcBuiltinMacro { builtin_name , helper_attrs , span : cx . attr_span }) } }
    };
}

impl_148!()