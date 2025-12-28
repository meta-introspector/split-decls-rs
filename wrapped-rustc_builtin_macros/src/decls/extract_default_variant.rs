macro_rules! deps {
    () => {
        MultipleDefaultsSugg!();
        MultipleDefaults!();
        NoDefaultVariant!();
        NonUnitDefault!();
        NoDefaultVariantSugg!();
        NonExhaustiveDefault!();
    };
}

macro_rules! extract_default_variant {
    () => {
        deps!();
        fn extract_default_variant < 'a > (cx : & ExtCtxt < '_ > , enum_def : & 'a EnumDef , trait_span : Span , item_span : Span ,) -> Result < & 'a rustc_ast :: Variant , ErrorGuaranteed > { let default_variants : SmallVec < [_ ; 1] > = enum_def . variants . iter () . filter (| variant | attr :: contains_name (& variant . attrs , kw :: Default)) . collect () ; let variant = match default_variants . as_slice () { [variant] => variant , [] => { let possible_defaults = enum_def . variants . iter () . filter (| variant | matches ! (variant . data , VariantData :: Unit (..))) . filter (| variant | ! attr :: contains_name (& variant . attrs , sym :: non_exhaustive)) ; let suggs = possible_defaults . map (| v | errors :: NoDefaultVariantSugg { span : v . span . shrink_to_lo () }) . collect () ; let guar = cx . dcx () . emit_err (errors :: NoDefaultVariant { span : trait_span , item_span , suggs }) ; return Err (guar) ; } [first , rest @ ..] => { let suggs = default_variants . iter () . filter_map (| variant | { let keep = attr :: find_by_name (& variant . attrs , kw :: Default) ? . span ; let spans : Vec < Span > = default_variants . iter () . flat_map (| v | { attr :: filter_by_name (& v . attrs , kw :: Default) . filter_map (| attr | (attr . span != keep) . then_some (attr . span)) }) . collect () ; (! spans . is_empty ()) . then_some (errors :: MultipleDefaultsSugg { spans , ident : variant . ident }) }) . collect () ; let guar = cx . dcx () . emit_err (errors :: MultipleDefaults { span : trait_span , first : first . span , additional : rest . iter () . map (| v | v . span) . collect () , suggs , }) ; return Err (guar) ; } } ; if cx . ecfg . features . default_field_values () && let VariantData :: Struct { fields , .. } = & variant . data && fields . iter () . all (| f | f . default . is_some ()) && ! fields . is_empty () { } else if ! matches ! (variant . data , VariantData :: Unit (..)) { let post = if cx . ecfg . features . default_field_values () { " or variants where every field has a default value" } else { "" } ; let guar = cx . dcx () . emit_err (errors :: NonUnitDefault { span : variant . ident . span , post }) ; return Err (guar) ; } if let Some (non_exhaustive_attr) = attr :: find_by_name (& variant . attrs , sym :: non_exhaustive) { let guar = cx . dcx () . emit_err (errors :: NonExhaustiveDefault { span : variant . ident . span , non_exhaustive : non_exhaustive_attr . span , }) ; return Err (guar) ; } Ok (variant) }
    };
}

extract_default_variant!()