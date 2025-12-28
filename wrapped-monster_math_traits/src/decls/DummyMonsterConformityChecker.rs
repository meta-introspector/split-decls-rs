macro_rules! deps {
    () => {
        MonsterConformityChecker!();
    };
}

macro_rules! DummyMonsterConformityChecker {
    () => {
        deps!();
        # [doc = " A dummy implementation of `MonsterConformityChecker` for testing."] # [derive (Debug , Default)] pub struct DummyMonsterConformityChecker ;
    };
}

DummyMonsterConformityChecker!();