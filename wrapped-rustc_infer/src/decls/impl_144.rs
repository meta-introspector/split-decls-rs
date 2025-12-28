macro_rules! deps {
    () => {
        GenericKind!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < 'tcx > GenericKind < 'tcx > { pub fn to_ty (& self , tcx : TyCtxt < 'tcx >) -> Ty < 'tcx > { match * self { GenericKind :: Param (ref p) => p . to_ty (tcx) , GenericKind :: Placeholder (ref p) => Ty :: new_placeholder (tcx , * p) , GenericKind :: Alias (ref p) => p . to_ty (tcx) , } } }
    };
}

impl_144!();