macro_rules! deps {
    () => {
        FieldDeclSpan!();
    };
}

macro_rules! FieldUniquenessCheckContext {
    () => {
        deps!();
        struct FieldUniquenessCheckContext < 'tcx > { tcx : TyCtxt < 'tcx > , seen_fields : FxIndexMap < Ident , FieldDeclSpan > , }
    };
}

FieldUniquenessCheckContext!();