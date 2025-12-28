macro_rules! OneTraitImplsBuilder {
    () => {
        # [derive (Default)] struct OneTraitImplsBuilder { non_blanket_impls : FxHashMap < SimplifiedType , Vec < ImplId > > , blanket_impls : Vec < ImplId > , }
    };
}

OneTraitImplsBuilder!();