macro_rules! deps {
    () => {
        FieldsShape!();
    };
}

macro_rules! adt_shape {
    () => {
        deps!();
        # [inline] fn adt_shape (adt_kind : ast :: StructKind) -> FieldsShape { match adt_kind { ast :: StructKind :: Record (_) => FieldsShape :: Record , ast :: StructKind :: Tuple (_) => FieldsShape :: Tuple , ast :: StructKind :: Unit => FieldsShape :: Unit , } }
    };
}

adt_shape!()