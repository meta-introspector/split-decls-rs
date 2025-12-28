macro_rules! AuxInner {
    () => {
        type AuxInner = Arc < dyn Any + Send + Sync + 'static > ;
    };
}

AuxInner!();