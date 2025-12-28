macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! GenericParamKind {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Debug , Eq , HashStable_Generic)] pub enum GenericParamKind { Type , Lifetime , Const , }
    };
}

GenericParamKind!();