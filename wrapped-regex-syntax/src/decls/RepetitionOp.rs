macro_rules! deps {
    () => {
        RepetitionKind!();
        Span!();
    };
}

macro_rules! RepetitionOp {
    () => {
        deps!();
        # [doc = " The repetition operator itself."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct RepetitionOp { # [doc = " The span of this operator. This includes things like `+`, `*?` and"] # [doc = " `{m,n}`."] pub span : Span , # [doc = " The type of operation."] pub kind : RepetitionKind , }
    };
}

RepetitionOp!()