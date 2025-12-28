macro_rules! OneTraitImpls {
    () => {
        # [derive (Debug , PartialEq)] struct OneTraitImpls { non_blanket_impls : FxHashMap < SimplifiedType , Box < [ImplId] > > , blanket_impls : Box < [ImplId] > , }
    };
}

OneTraitImpls!()