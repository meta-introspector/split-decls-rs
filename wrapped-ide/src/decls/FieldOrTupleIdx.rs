macro_rules! FieldOrTupleIdx {
    () => {
        # [derive (Copy , Clone)] enum FieldOrTupleIdx { Field (Field) , TupleIdx (usize) , }
    };
}

FieldOrTupleIdx!();