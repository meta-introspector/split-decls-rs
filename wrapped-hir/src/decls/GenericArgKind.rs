macro_rules! deps {
    () => {
        Type!();
        Const!();
    };
}

macro_rules! GenericArgKind {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum GenericArgKind { Lifetime , Type , Const , }
    };
}

GenericArgKind!()