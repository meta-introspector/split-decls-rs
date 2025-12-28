macro_rules! deps {
    () => {
        PointerKind!();
    };
}

macro_rules! CastError {
    () => {
        deps!();
        # [derive (Debug)] enum CastError < 'tcx > { ErrorGuaranteed (ErrorGuaranteed) , CastToBool , CastToChar , DifferingKinds { src_kind : PointerKind < 'tcx > , dst_kind : PointerKind < 'tcx > , } , # [doc = " Cast of thin to wide raw ptr (e.g., `*const () as *const [u8]`)."] SizedUnsizedCast , IllegalCast , NeedDeref , NeedViaPtr , NeedViaThinPtr , NeedViaInt , NonScalar , UnknownExprPtrKind , UnknownCastPtrKind , # [doc = " Cast of int to (possibly) wide raw pointer."] # [doc = ""] # [doc = " Argument is the specific name of the metadata in plain words, such as \"a vtable\""] # [doc = " or \"a length\". If this argument is None, then the metadata is unknown, for example,"] # [doc = " when we're typechecking a type parameter with a ?Sized bound."] IntToWideCast (Option < & 'static str >) , ForeignNonExhaustiveAdt , PtrPtrAddingAutoTrait (Vec < DefId >) , }
    };
}

CastError!();