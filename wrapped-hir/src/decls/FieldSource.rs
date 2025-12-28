macro_rules! deps {
    () => {
        TupleField!();
    };
}

macro_rules! FieldSource {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub enum FieldSource { Named (ast :: RecordField) , Pos (ast :: TupleField) , }
    };
}

FieldSource!()