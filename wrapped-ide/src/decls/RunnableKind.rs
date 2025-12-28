macro_rules! deps {
    () => {
        TestId!();
        TestAttr!();
    };
}

macro_rules! RunnableKind {
    () => {
        deps!();
        # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub enum RunnableKind { TestMod { path : String } , Test { test_id : TestId , attr : TestAttr } , Bench { test_id : TestId } , DocTest { test_id : TestId } , Bin , }
    };
}

RunnableKind!();