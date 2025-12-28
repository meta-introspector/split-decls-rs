macro_rules! deps {
    () => {
        ExpressionStore!();
    };
}

macro_rules! ImplSignature {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct ImplSignature { pub generic_params : Arc < GenericParams > , pub store : Arc < ExpressionStore > , pub self_ty : TypeRefId , pub target_trait : Option < TraitRef > , pub flags : ImplFlags , }
    };
}

ImplSignature!();