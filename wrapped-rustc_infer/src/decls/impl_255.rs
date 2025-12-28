macro_rules! deps {
    () => {
        ValuePairs!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl < 'tcx > ValuePairs < 'tcx > { pub fn ty (& self) -> Option < (Ty < 'tcx > , Ty < 'tcx >) > { if let ValuePairs :: Terms (ExpectedFound { expected , found }) = self && let Some (expected) = expected . as_type () && let Some (found) = found . as_type () { Some ((expected , found)) } else { None } } }
    };
}

impl_255!();