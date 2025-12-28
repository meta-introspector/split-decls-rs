macro_rules! TyCtxtConsts {
    () => {
        # [doc = " Associate some local constants with the `'tcx` lifetime"] struct TyCtxtConsts < 'tcx > (PhantomData < & 'tcx () >) ;
    };
}

TyCtxtConsts!()