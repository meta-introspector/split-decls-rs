macro_rules! deps {
    () => {
        ReferenceConversionType!();
    };
}

macro_rules! ReferenceConversion {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct ReferenceConversion < 'db > { conversion : ReferenceConversionType , ty : hir :: Type < 'db > , impls_deref : bool , }
    };
}

ReferenceConversion!();