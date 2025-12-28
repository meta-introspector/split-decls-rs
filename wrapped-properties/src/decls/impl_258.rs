macro_rules! deps {
    () => {
        PropertyEnumToValueNameLookup!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl PropertyEnumToValueNameLookup for PropertyScriptToIcuScriptMap < '_ > { fn get (& self , prop : u32) -> Option < & str > { self . map . get_ule_ref (usize :: try_from (prop) . ok () ?) . and_then (| no | no . as_ref ()) . map (| s | s . as_str ()) } }
    };
}

impl_258!()