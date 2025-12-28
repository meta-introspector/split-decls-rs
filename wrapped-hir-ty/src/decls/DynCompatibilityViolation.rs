macro_rules! deps {
    () => {
        MethodViolationCode!();
    };
}

macro_rules! DynCompatibilityViolation {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum DynCompatibilityViolation { SizedSelf , SelfReferential , Method (FunctionId , MethodViolationCode) , AssocConst (ConstId) , GAT (TypeAliasId) , HasNonCompatibleSuperTrait (TraitId) , }
    };
}

DynCompatibilityViolation!();