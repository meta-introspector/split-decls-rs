macro_rules! deps {
    () => {
        ReceiverValidityError!();
    };
}

macro_rules! confirm_type_is_not_a_method_generic_param {
    () => {
        deps!();
        # [doc = " Confirms that a type is not a type parameter referring to one of the"] # [doc = " method's type params."] fn confirm_type_is_not_a_method_generic_param (ty : Ty < '_ > , method_generics : & ty :: Generics ,) -> Result < () , ReceiverValidityError > { if let ty :: Param (param) = ty . kind () { if (param . index as usize) >= method_generics . parent_count { return Err (ReceiverValidityError :: MethodGenericParamUsed) ; } } Ok (()) }
    };
}

confirm_type_is_not_a_method_generic_param!();