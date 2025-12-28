macro_rules! deps {
    () => {
        SemanticHasher!();
    };
}

macro_rules! DummySemanticHasher {
    () => {
        deps!();
        # [doc = " A dummy implementation of `SemanticHasher` for testing and initial development."] # [derive (Debug , Default)] pub struct DummySemanticHasher ;
    };
}

DummySemanticHasher!()