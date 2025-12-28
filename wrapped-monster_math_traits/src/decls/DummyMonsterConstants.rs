macro_rules! deps {
    () => {
        MonsterConstants!();
    };
}

macro_rules! DummyMonsterConstants {
    () => {
        deps!();
        # [doc = " A dummy implementation of `MonsterConstants` for testing."] # [derive (Debug , Default , Clone , PartialEq , Serialize , Deserialize)] pub struct DummyMonsterConstants ;
    };
}

DummyMonsterConstants!()