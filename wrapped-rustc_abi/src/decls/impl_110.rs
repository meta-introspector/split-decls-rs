macro_rules! deps {
    () => {
        FieldsShape!();
        LayoutData!();
        Variants!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < FieldIdx : Idx , VariantIdx : Idx > fmt :: Debug for LayoutData < FieldIdx , VariantIdx > where FieldsShape < FieldIdx > : fmt :: Debug , Variants < FieldIdx , VariantIdx > : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let LayoutData { size , align , backend_repr , fields , largest_niche , uninhabited , variants , max_repr_align , unadjusted_abi_align , randomization_seed , } = self ; f . debug_struct ("Layout") . field ("size" , size) . field ("align" , align) . field ("backend_repr" , backend_repr) . field ("fields" , fields) . field ("largest_niche" , largest_niche) . field ("uninhabited" , uninhabited) . field ("variants" , variants) . field ("max_repr_align" , max_repr_align) . field ("unadjusted_abi_align" , unadjusted_abi_align) . field ("randomization_seed" , randomization_seed) . finish () } }
    };
}

impl_110!();