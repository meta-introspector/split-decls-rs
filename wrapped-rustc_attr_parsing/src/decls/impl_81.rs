macro_rules! deps {
    () => {
        AllowedTargets!();
        Stage!();
        OnDuplicate!();
        ArgParser!();
        SingleAttributeParser!();
        IllFormedAttributeInput!();
        InlineParser!();
        AttributeOrder!();
        AcceptContext!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for InlineParser { const PATH : & 'static [Symbol] = & [sym :: inline] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: WarnButFutureError ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Allow (Target :: Closure) , Allow (Target :: Delegation { mac : false }) , Warn (Target :: Method (MethodKind :: Trait { body : false })) , Warn (Target :: ForeignFn) , Warn (Target :: Field) , Warn (Target :: MacroDef) , Warn (Target :: Arm) , Warn (Target :: AssocConst) , Warn (Target :: MacroCall) ,]) ; const TEMPLATE : AttributeTemplate = template ! (Word , List : & ["always" , "never"] , "https://doc.rust-lang.org/reference/attributes/codegen.html#the-inline-attribute") ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { match args { ArgParser :: NoArgs => Some (AttributeKind :: Inline (InlineAttr :: Hint , cx . attr_span)) , ArgParser :: List (list) => { let Some (l) = list . single () else { cx . expected_single_argument (list . span) ; return None ; } ; match l . meta_item () . and_then (| i | i . path () . word_sym ()) { Some (sym :: always) => { Some (AttributeKind :: Inline (InlineAttr :: Always , cx . attr_span)) } Some (sym :: never) => { Some (AttributeKind :: Inline (InlineAttr :: Never , cx . attr_span)) } _ => { cx . expected_specific_argument (l . span () , & [sym :: always , sym :: never]) ; return None ; } } } ArgParser :: NameValue (_) => { let suggestions = < Self as SingleAttributeParser < S > > :: TEMPLATE . suggestions (cx . attr_style , "inline") ; let span = cx . attr_span ; cx . emit_lint (AttributeLintKind :: IllFormedAttributeInput { suggestions } , span) ; return None ; } } } }
    };
}

impl_81!()