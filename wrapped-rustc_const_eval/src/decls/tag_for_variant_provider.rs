macro_rules! deps {
    () => {
        InterpCx!();
        DummyMachine!();
    };
}

macro_rules! tag_for_variant_provider {
    () => {
        deps!();
        # [doc = " Computes the tag (if any) for a given type and variant."] # [instrument (skip (tcx) , level = "debug")] pub fn tag_for_variant_provider < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , (Ty < 'tcx > , VariantIdx) > ,) -> Option < ty :: ScalarInt > { let (ty , variant_index) = key . value ; assert ! (ty . is_enum ()) ; let ecx = InterpCx :: new (tcx , DUMMY_SP , key . typing_env , crate :: const_eval :: DummyMachine) ; let layout = ecx . layout_of (ty) . unwrap () ; ecx . tag_for_variant (layout , variant_index) . unwrap () . map (| (tag , _tag_field) | tag) }
    };
}

tag_for_variant_provider!()