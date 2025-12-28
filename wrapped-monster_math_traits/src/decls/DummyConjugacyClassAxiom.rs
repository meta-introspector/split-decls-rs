macro_rules! deps {
    () => {
        ConjugacyClassAxiom!();
    };
}

macro_rules! DummyConjugacyClassAxiom {
    () => {
        deps!();
        # [doc = " A dummy implementation of `ConjugacyClassAxiom` for testing."] # [derive (Debug , Default)] pub struct DummyConjugacyClassAxiom ;
    };
}

DummyConjugacyClassAxiom!()