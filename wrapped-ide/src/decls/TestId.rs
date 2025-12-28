macro_rules! TestId {
    () => {
        # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub enum TestId { Name (SmolStr) , Path (String) , }
    };
}

TestId!()