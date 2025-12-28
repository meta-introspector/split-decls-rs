macro_rules! deps {
    () => {
        MonsterFactorsAxiom!();
    };
}

macro_rules! DummyMonsterFactorsAxiom {
    () => {
        deps!();
        # [doc = " A dummy implementation of `MonsterFactorsAxiom` for testing."] # [derive (Debug , Default)] pub struct DummyMonsterFactorsAxiom ;
    };
}

DummyMonsterFactorsAxiom!()