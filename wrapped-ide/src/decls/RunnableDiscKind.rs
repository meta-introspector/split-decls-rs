macro_rules! RunnableDiscKind {
    () => {
        # [derive (Debug , Clone , Hash , PartialEq , Eq , PartialOrd , Ord)] enum RunnableDiscKind { TestMod , Test , DocTest , Bench , Bin , }
    };
}

RunnableDiscKind!();