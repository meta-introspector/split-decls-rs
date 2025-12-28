macro_rules! deps {
    () => {
        Param!();
        Lifetime!();
    };
}

macro_rules! PreciseCapturingArgKind {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic , Encodable , Decodable)] pub enum PreciseCapturingArgKind < T , U > { Lifetime (T) , # [doc = " Non-lifetime argument (type or const)"] Param (U) , }
    };
}

PreciseCapturingArgKind!();