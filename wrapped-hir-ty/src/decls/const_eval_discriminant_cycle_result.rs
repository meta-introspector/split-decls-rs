macro_rules! deps {
    () => {
        ConstEvalError!();
        HirDatabase!();
    };
}

macro_rules! const_eval_discriminant_cycle_result {
    () => {
        deps!();
        pub (crate) fn const_eval_discriminant_cycle_result < 'db > (_db : & 'db dyn HirDatabase , _salsa_id : salsa :: Id , _enum_variant_id : EnumVariantId ,) -> Result < i128 , ConstEvalError < 'db > > { Err (ConstEvalError :: MirLowerError (MirLowerError :: Loop)) }
    };
}

const_eval_discriminant_cycle_result!()