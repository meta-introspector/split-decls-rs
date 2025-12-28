macro_rules! deps {
    () => {
        PatId!();
    };
}

macro_rules! RecordFieldPat {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct RecordFieldPat { pub name : Name , pub pat : PatId , }
    };
}

RecordFieldPat!()