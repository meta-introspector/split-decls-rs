macro_rules! deps {
    () => {
        ConstVariableValue!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < 'tcx > ConstVariableValue < 'tcx > { # [doc = " If this value is known, returns the const it is known to be."] # [doc = " Otherwise, `None`."] pub (crate) fn known (& self) -> Option < ty :: Const < 'tcx > > { match * self { ConstVariableValue :: Unknown { .. } => None , ConstVariableValue :: Known { value } => Some (value) , } } }
    };
}

impl_241!();