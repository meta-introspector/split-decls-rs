macro_rules! deps {
    () => {
        ReferenceConversion!();
    };
}

macro_rules! convert_reference_type {
    () => {
        deps!();
        pub (crate) fn convert_reference_type < 'db > (ty : hir :: Type < 'db > , db : & 'db RootDatabase , famous_defs : & FamousDefs < '_ , 'db > ,) -> Option < ReferenceConversion < 'db > > { handle_copy (& ty , db) . or_else (| | handle_as_ref_str (& ty , db , famous_defs)) . or_else (| | handle_as_ref_slice (& ty , db , famous_defs)) . or_else (| | handle_dereferenced (& ty , db , famous_defs)) . or_else (| | handle_option_as_ref (& ty , db , famous_defs)) . or_else (| | handle_result_as_ref (& ty , db , famous_defs)) . map (| (conversion , impls_deref) | ReferenceConversion { ty , conversion , impls_deref }) }
    };
}

convert_reference_type!();