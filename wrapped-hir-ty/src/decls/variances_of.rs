macro_rules! deps {
    () => {
        Context!();
        HirDatabase!();
    };
}

macro_rules! variances_of {
    () => {
        deps!();
        pub (crate) fn variances_of (db : & dyn HirDatabase , def : GenericDefId) -> VariancesOf < '_ > { tracing :: debug ! ("variances_of(def={:?})" , def) ; let interner = DbInterner :: new_with (db , None , None) ; match def { GenericDefId :: FunctionId (_) => () , GenericDefId :: AdtId (adt) => { if let AdtId :: StructId (id) = adt { let flags = & db . struct_signature (id) . flags ; if flags . contains (StructFlags :: IS_UNSAFE_CELL) { return VariancesOf :: new_from_iter (interner , [Variance :: Invariant]) ; } else if flags . contains (StructFlags :: IS_PHANTOM_DATA) { return VariancesOf :: new_from_iter (interner , [Variance :: Covariant]) ; } } } _ => return VariancesOf :: new_from_iter (interner , []) , } let generics = generics (db , def) ; let count = generics . len () ; if count == 0 { return VariancesOf :: new_from_iter (interner , []) ; } let mut variances = Context { generics , variances : vec ! [Variance :: Bivariant ; count] , db } . solve () ; for variance in & mut variances { if * variance == Variance :: Bivariant { * variance = Variance :: Invariant ; } } VariancesOf :: new_from_iter (interner , variances) }
    };
}

variances_of!()