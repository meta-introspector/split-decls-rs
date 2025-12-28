macro_rules! deps {
    () => {
        InferredIndex!();
        VarianceTermPtr!();
    };
}

macro_rules! VarianceTerm {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub (crate) enum VarianceTerm < 'a > { ConstantTerm (ty :: Variance) , TransformTerm (VarianceTermPtr < 'a > , VarianceTermPtr < 'a >) , InferredTerm (InferredIndex) , }
    };
}

VarianceTerm!();