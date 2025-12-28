macro_rules! ensure_monomorphic_enough {
    () => {
        # [doc = " Checks whether a type contains generic parameters which must be instantiated."] # [doc = ""] # [doc = " In case it does, returns a `TooGeneric` const eval error."] pub (crate) fn ensure_monomorphic_enough < 'tcx , T > (_tcx : TyCtxt < 'tcx > , ty : T) -> InterpResult < 'tcx > where T : TypeVisitable < TyCtxt < 'tcx > > , { debug ! ("ensure_monomorphic_enough: ty={:?}" , ty) ; if ty . has_param () { throw_inval ! (TooGeneric) ; } interp_ok (()) }
    };
}

ensure_monomorphic_enough!();