macro_rules! deps {
    () => {
        InferredCaptureInformation!();
    };
}

macro_rules! InferBorrowKind {
    () => {
        deps!();
        struct InferBorrowKind < 'tcx > { closure_def_id : LocalDefId , # [doc = " For each Place that is captured by the closure, we track the minimal kind of"] # [doc = " access we need (ref, ref mut, move, etc) and the expression that resulted in such access."] # [doc = ""] # [doc = " Consider closure where s.str1 is captured via an ImmutableBorrow and"] # [doc = " s.str2 via a MutableBorrow"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " struct SomeStruct { str1: String, str2: String };"] # [doc = ""] # [doc = " // Assume that the HirId for the variable definition is `V1`"] # [doc = " let mut s = SomeStruct { str1: format!(\"s1\"), str2: format!(\"s2\") };"] # [doc = ""] # [doc = " let fix_s = |new_s2| {"] # [doc = "     // Assume that the HirId for the expression `s.str1` is `E1`"] # [doc = "     println!(\"Updating SomeStruct with str1={0}\", s.str1);"] # [doc = "     // Assume that the HirId for the expression `*s.str2` is `E2`"] # [doc = "     s.str2 = new_s2;"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " For closure `fix_s`, (at a high level) the map contains"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " Place { V1, [ProjectionKind::Field(Index=0, Variant=0)] } : CaptureKind { E1, ImmutableBorrow }"] # [doc = " Place { V1, [ProjectionKind::Field(Index=1, Variant=0)] } : CaptureKind { E2, MutableBorrow }"] # [doc = " ```"] capture_information : InferredCaptureInformation < 'tcx > , fake_reads : Vec < (Place < 'tcx > , FakeReadCause , HirId) > , }
    };
}

InferBorrowKind!();