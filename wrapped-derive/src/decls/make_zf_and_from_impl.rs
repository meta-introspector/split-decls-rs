macro_rules! deps {
    () => {
        UnsizedFields!();
        FieldInfo!();
    };
}

macro_rules! make_zf_and_from_impl {
    () => {
        deps!();
        # [expect (clippy :: too_many_arguments)] fn make_zf_and_from_impl (sized_fields : & [FieldInfo] , unsized_field_info : & UnsizedFields , fields : & Fields , name : & Ident , ule_name : & Ident , maybe_lt : Option < & Lifetime > , span : Span , skip_from : bool ,) -> TokenStream2 { if ! unsized_field_info . has_zf () { return quote ! () ; } let lt = if let Some (ref lt) = maybe_lt { lt } else { return Error :: new (span , "Can only generate ZeroFrom impls for types with lifetimes" ,) . to_compile_error () ; } ; let mut field_inits = sized_fields . iter () . map (| f | { let ty = & f . field . ty ; let accessor = & f . accessor ; let setter = f . setter () ; quote ! (# setter <# ty as zerovec :: ule :: AsULE >:: from_unaligned (other .# accessor)) }) . collect :: < Vec < _ > > () ; unsized_field_info . push_zf_setters (lt , & mut field_inits) ; let field_inits = utils :: wrap_field_inits (& field_inits , fields) ; let zerofrom_trait = quote ! (zerovec :: __zerovec_internal_reexport :: ZeroFrom) ; let maybe_from = if skip_from { quote ! () } else { quote ! (impl <# lt > From <&# lt # ule_name > for # name <# lt > { fn from (other : &# lt # ule_name) -> Self { < Self as # zerofrom_trait <# lt , # ule_name >>:: zero_from (other) } }) } ; quote ! (impl <# lt > # zerofrom_trait <# lt , # ule_name > for # name <# lt > { fn zero_from (other : &# lt # ule_name) -> Self { Self # field_inits } } # maybe_from) }
    };
}

make_zf_and_from_impl!()