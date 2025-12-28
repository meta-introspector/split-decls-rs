macro_rules! ClosureReturnTypeHints {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub enum ClosureReturnTypeHints { Always , WithBlock , Never , }
    };
}

ClosureReturnTypeHints!()