macro_rules! DerivedFieldMetadata {
    () => {
        # [derive (Debug)] struct DerivedFieldMetadata { ident : Ident , into : Type , owned : Option < bool > , with : Option < Path > , }
    };
}

DerivedFieldMetadata!();