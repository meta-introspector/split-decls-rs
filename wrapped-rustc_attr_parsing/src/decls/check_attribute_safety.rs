macro_rules! deps {
    () => {
        UnsafeAttrOutsideUnsafeSuggestion!();
        InvalidAttrUnsafe!();
        UnsafeAttrOutsideUnsafe!();
    };
}

macro_rules! check_attribute_safety {
    () => {
        deps!();
        pub fn check_attribute_safety (psess : & ParseSess , builtin_attr_safety : Option < AttributeSafety > , attr : & Attribute , id : NodeId ,) { let attr_item = attr . get_normal_item () ; match (builtin_attr_safety , attr_item . unsafety) { (Some (AttributeSafety :: Unsafe { .. }) , Safety :: Unsafe (..)) => { } (Some (AttributeSafety :: Unsafe { unsafe_since }) , Safety :: Default) => { let path_span = attr_item . path . span ; let diag_span = attr_item . span () ; let emit_error = match unsafe_since { None => true , Some (unsafe_since) => path_span . edition () >= unsafe_since , } ; if emit_error { psess . dcx () . emit_err (errors :: UnsafeAttrOutsideUnsafe { span : path_span , suggestion : errors :: UnsafeAttrOutsideUnsafeSuggestion { left : diag_span . shrink_to_lo () , right : diag_span . shrink_to_hi () , } , }) ; } else { psess . buffer_lint (UNSAFE_ATTR_OUTSIDE_UNSAFE , path_span , id , BuiltinLintDiag :: UnsafeAttrOutsideUnsafe { attribute_name_span : path_span , sugg_spans : (diag_span . shrink_to_lo () , diag_span . shrink_to_hi ()) , } ,) ; } } (Some (AttributeSafety :: Normal) | None , Safety :: Unsafe (unsafe_span)) => { psess . dcx () . emit_err (errors :: InvalidAttrUnsafe { span : unsafe_span , name : attr_item . path . clone () , }) ; } (Some (AttributeSafety :: Normal) , Safety :: Default) => { } (None , Safety :: Default) => { } (Some (AttributeSafety :: Unsafe { .. } | AttributeSafety :: Normal) | None , Safety :: Safe (..) ,) => { psess . dcx () . span_delayed_bug (attr_item . span () , "`check_attribute_safety` does not expect `Safety::Safe` on attributes" ,) ; } } }
    };
}

check_attribute_safety!();