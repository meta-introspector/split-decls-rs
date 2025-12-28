macro_rules! deps {
    () => {
        OnDuplicate!();
        Stage!();
        NullOnLinkSection!();
        AllowedTargets!();
        SingleAttributeParser!();
        AttributeOrder!();
        LinkSectionParser!();
        AcceptContext!();
        ArgParser!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for LinkSectionParser { const PATH : & [Symbol] = & [sym :: link_section] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: WarnButFutureError ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowListWarnRest (& [Allow (Target :: Static) , Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : false })) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) ,]) ; const TEMPLATE : AttributeTemplate = template ! (NameValueStr : "name" , "https://doc.rust-lang.org/reference/abi.html#the-link_section-attribute") ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let Some (nv) = args . name_value () else { cx . expected_name_value (cx . attr_span , None) ; return None ; } ; let Some (name) = nv . value_as_str () else { cx . expected_string_literal (nv . value_span , Some (nv . value_as_lit ())) ; return None ; } ; if name . as_str () . contains ('\0') { cx . emit_err (NullOnLinkSection { span : cx . attr_span }) ; return None ; } Some (LinkSection { name , span : cx . attr_span }) } }
    };
}

impl_91!()