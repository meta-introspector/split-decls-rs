macro_rules! AnyExpectations {
    () => {
        # [doc (hidden)] pub trait AnyExpectations : Any + Send + Sync { }
    };
}

AnyExpectations!()