macro_rules! RunnableKind {
    () => {
        # [doc = " The kind of runnable."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum RunnableKind { Check , # [doc = " Can run a binary."] Run , # [doc = " Run a single test."] TestOne , }
    };
}

RunnableKind!()