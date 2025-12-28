macro_rules! deps {
    () => {
        IgnoreParser!();
        AcceptContext!();
        AttributeOrder!();
        OnDuplicate!();
        IllFormedAttributeInput!();
        Stage!();
        ArgParser!();
        SingleAttributeParser!();
        AllowedTargets!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for IgnoreParser { const PATH : & [Symbol] = & [sym :: ignore] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowListWarnRest (& [Allow (Target :: Fn) , Error (Target :: WherePredicate)]) ; const TEMPLATE : AttributeTemplate = template ! (Word , NameValueStr : "reason" , "https://doc.rust-lang.org/reference/attributes/testing.html#the-ignore-attribute") ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { Some (AttributeKind :: Ignore { span : cx . attr_span , reason : match args { ArgParser :: NoArgs => None , ArgParser :: NameValue (name_value) => { let Some (str_value) = name_value . value_as_str () else { let suggestions = < Self as SingleAttributeParser < S > > :: TEMPLATE . suggestions (cx . attr_style , "ignore") ; let span = cx . attr_span ; cx . emit_lint (AttributeLintKind :: IllFormedAttributeInput { suggestions } , span ,) ; return None ; } ; Some (str_value) } ArgParser :: List (_) => { let suggestions = < Self as SingleAttributeParser < S > > :: TEMPLATE . suggestions (cx . attr_style , "ignore") ; let span = cx . attr_span ; cx . emit_lint (AttributeLintKind :: IllFormedAttributeInput { suggestions } , span) ; return None ; } } , }) } }
    };
}

impl_199!();