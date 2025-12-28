macro_rules! deps {
    () => {
        TypeVariableValue!();
    };
}

macro_rules! TyVidEqKey {
    () => {
        deps!();
        # [doc = " These structs (a newtyped TyVid) are used as the unification key"] # [doc = " for the `eq_relations`; they carry a `TypeVariableValue` along"] # [doc = " with them."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) struct TyVidEqKey < 'tcx > { vid : ty :: TyVid , phantom : PhantomData < TypeVariableValue < 'tcx > > , }
    };
}

TyVidEqKey!();