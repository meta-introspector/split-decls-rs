macro_rules! deps {
    () => {
        MissingLifetimeKind!();
    };
}

macro_rules! LifetimeParamKind {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , HashStable_Generic)] pub enum LifetimeParamKind { Explicit , Elided (MissingLifetimeKind) , Error , }
    };
}

LifetimeParamKind!();