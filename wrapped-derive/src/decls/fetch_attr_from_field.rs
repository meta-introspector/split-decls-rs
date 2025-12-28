macro_rules! fetch_attr_from_field {
    () => {
        fn fetch_attr_from_field (field : & Field) -> Result < Option < & Attribute > > { let found_attributes : Vec < _ > = field . attrs . iter () . filter (| a | { let path = a . path () ; let name = quote ! (# path) . to_string () ; name == ARBITRARY_ATTRIBUTE_NAME }) . collect () ; if found_attributes . len () > 1 { let name = field . ident . as_ref () . unwrap () ; let msg = format ! ("Multiple conflicting #[{ARBITRARY_ATTRIBUTE_NAME}] attributes found on field `{name}`") ; return Err (syn :: Error :: new (field . span () , msg)) ; } Ok (found_attributes . into_iter () . next ()) }
    };
}

fetch_attr_from_field!();