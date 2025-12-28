macro_rules! deps {
    () => {
        AttributeOrder!();
        SingleAttributeParser!();
        Stage!();
        ExportNameParser!();
        AcceptContext!();
        AllowedTargets!();
        OnDuplicate!();
        ArgParser!();
        NullOnExport!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for ExportNameParser { const PATH : & [rustc_span :: Symbol] = & [sym :: export_name] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: WarnButFutureError ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Static) , Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Warn (Target :: Field) , Warn (Target :: Arm) , Warn (Target :: MacroDef) , Warn (Target :: MacroCall) ,]) ; const TEMPLATE : AttributeTemplate = template ! (NameValueStr : "name") ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let Some (nv) = args . name_value () else { cx . expected_name_value (cx . attr_span , None) ; return None ; } ; let Some (name) = nv . value_as_str () else { cx . expected_string_literal (nv . value_span , Some (nv . value_as_lit ())) ; return None ; } ; if name . as_str () . contains ('\0') { cx . emit_err (NullOnExport { span : cx . attr_span }) ; return None ; } Some (AttributeKind :: ExportName { name , span : cx . attr_span }) } }
    };
}

impl_37!()