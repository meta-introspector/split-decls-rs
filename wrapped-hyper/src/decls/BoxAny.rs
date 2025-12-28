macro_rules! deps {
    () => {
        AsTaskType!();
    };
}

macro_rules! BoxAny {
    () => {
        deps!();
        type BoxAny = Box < dyn AsTaskType + Send + Sync > ;
    };
}

BoxAny!()