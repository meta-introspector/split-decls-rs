macro_rules! deps {
    () => {
        ExpressionStore!();
    };
}

macro_rules! FunctionSignature {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub struct FunctionSignature { pub name : Name , pub generic_params : Arc < GenericParams > , pub store : Arc < ExpressionStore > , pub params : Box < [TypeRefId] > , pub ret_type : Option < TypeRefId > , pub abi : Option < Symbol > , pub flags : FnFlags , pub legacy_const_generics_indices : Option < Box < Box < [u32] > > > , }
    };
}

FunctionSignature!()