macro_rules! deps {
    () => {
        OneTraitImplsBuilder!();
        OneTraitImpls!();
    };
}

macro_rules! impl_776 {
    () => {
        deps!();
        impl OneTraitImplsBuilder { fn finish (self) -> OneTraitImpls { let mut non_blanket_impls = self . non_blanket_impls . into_iter () . map (| (self_ty , impls) | (self_ty , impls . into_boxed_slice ())) . collect :: < FxHashMap < _ , _ > > () ; non_blanket_impls . shrink_to_fit () ; let blanket_impls = self . blanket_impls . into_boxed_slice () ; OneTraitImpls { non_blanket_impls , blanket_impls } } }
    };
}

impl_776!();