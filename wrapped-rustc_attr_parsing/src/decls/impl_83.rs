macro_rules! deps {
    () => {
        AllowedTargets!();
        OnDuplicate!();
        AttributeOrder!();
        Stage!();
        SingleAttributeParser!();
        AcceptContext!();
        ArgParser!();
        RustcForceInlineParser!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for RustcForceInlineParser { const PATH : & 'static [Symbol] = & [sym :: rustc_force_inline] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: WarnButFutureError ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn)]) ; const TEMPLATE : AttributeTemplate = template ! (Word , List : & ["reason"] , NameValueStr : "reason") ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let reason = match args { ArgParser :: NoArgs => None , ArgParser :: List (list) => { let Some (l) = list . single () else { cx . expected_single_argument (list . span) ; return None ; } ; let Some (reason) = l . lit () . and_then (| i | i . kind . str ()) else { cx . expected_string_literal (l . span () , l . lit ()) ; return None ; } ; Some (reason) } ArgParser :: NameValue (v) => { let Some (reason) = v . value_as_str () else { cx . expected_string_literal (v . value_span , Some (v . value_as_lit ())) ; return None ; } ; Some (reason) } } ; Some (AttributeKind :: Inline (InlineAttr :: Force { attr_span : cx . attr_span , reason } , cx . attr_span ,)) } }
    };
}

impl_83!()