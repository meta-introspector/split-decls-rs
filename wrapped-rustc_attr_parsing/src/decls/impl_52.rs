macro_rules! deps {
    () => {
        SanitizeParser!();
        AllowedTargets!();
        AcceptContext!();
        OnDuplicate!();
        ArgParser!();
        SingleAttributeParser!();
        Stage!();
        AttributeOrder!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for SanitizeParser { const PATH : & [Symbol] = & [sym :: sanitize] ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const TEMPLATE : AttributeTemplate = template ! (List : & [r#"address = "on|off""# , r#"kernel_address = "on|off""# , r#"cfi = "on|off""# , r#"hwaddress = "on|off""# , r#"kcfi = "on|off""# , r#"memory = "on|off""# , r#"memtag = "on|off""# , r#"shadow_call_stack = "on|off""# , r#"thread = "on|off""#]) ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let Some (list) = args . list () else { cx . expected_list (cx . attr_span) ; return None ; } ; let mut on_set = SanitizerSet :: empty () ; let mut off_set = SanitizerSet :: empty () ; for item in list . mixed () { let Some (item) = item . meta_item () else { cx . expected_name_value (item . span () , None) ; continue ; } ; let path = item . path () . word_sym () ; let Some (value) = item . args () . name_value () else { cx . expected_name_value (item . span () , path) ; continue ; } ; let mut apply = | s : SanitizerSet | { let is_on = match value . value_as_str () { Some (sym :: on) => true , Some (sym :: off) => false , Some (_) => { cx . expected_specific_argument_strings (value . value_span , & [sym :: on , sym :: off] ,) ; return ; } None => { cx . expected_string_literal (value . value_span , Some (value . value_as_lit ())) ; return ; } } ; if is_on { on_set |= s ; } else { off_set |= s ; } } ; match path { Some (sym :: address) | Some (sym :: kernel_address) => { apply (SanitizerSet :: ADDRESS | SanitizerSet :: KERNELADDRESS) } Some (sym :: cfi) => apply (SanitizerSet :: CFI) , Some (sym :: kcfi) => apply (SanitizerSet :: KCFI) , Some (sym :: memory) => apply (SanitizerSet :: MEMORY) , Some (sym :: memtag) => apply (SanitizerSet :: MEMTAG) , Some (sym :: shadow_call_stack) => apply (SanitizerSet :: SHADOWCALLSTACK) , Some (sym :: thread) => apply (SanitizerSet :: THREAD) , Some (sym :: hwaddress) => apply (SanitizerSet :: HWADDRESS) , _ => { cx . expected_specific_argument_strings (item . path () . span () , & [sym :: address , sym :: cfi , sym :: kcfi , sym :: memory , sym :: memtag , sym :: shadow_call_stack , sym :: thread , sym :: hwaddress ,] ,) ; continue ; } } } Some (AttributeKind :: Sanitize { on_set , off_set , span : cx . attr_span }) } }
    };
}

impl_52!()