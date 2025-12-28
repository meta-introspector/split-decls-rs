macro_rules! deps {
    () => {
        ValuePairs!();
    };
}

macro_rules! TypeTrace {
    () => {
        deps!();
        # [doc = " The trace designates the path through inference that we took to"] # [doc = " encounter an error or subtyping constraint."] # [doc = ""] # [doc = " See the `error_reporting` module for more details."] # [derive (Clone , Debug)] pub struct TypeTrace < 'tcx > { pub cause : ObligationCause < 'tcx > , pub values : ValuePairs < 'tcx > , }
    };
}

TypeTrace!();