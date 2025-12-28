macro_rules! deps {
    () => {
        FullTypeResolver!();
        FixupResult!();
        InferCtxt!();
    };
}

macro_rules! fully_resolve {
    () => {
        deps!();
        # [doc = " Full type resolution replaces all type and region variables with"] # [doc = " their concrete results. If any variable cannot be replaced (never unified, etc)"] # [doc = " then an `Err` result is returned."] pub fn fully_resolve < 'tcx , T > (infcx : & InferCtxt < 'tcx > , value : T) -> FixupResult < T > where T : TypeFoldable < TyCtxt < 'tcx > > , { value . try_fold_with (& mut FullTypeResolver { infcx }) }
    };
}

fully_resolve!()