macro_rules! PagerankError {
    () => {
        # [derive (Debug)] pub enum PagerankError { CapacityError (String) , }
    };
}

PagerankError!()