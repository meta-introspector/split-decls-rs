macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! AngleBrackets {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , PartialEq , Eq , HashStable_Generic)] pub enum AngleBrackets { # [doc = " E.g. `Path`."] Missing , # [doc = " E.g. `Path<>`."] Empty , # [doc = " E.g. `Path<T>`."] Full , }
    };
}

AngleBrackets!();