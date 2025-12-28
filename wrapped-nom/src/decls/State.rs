macro_rules! deps {
    () => {
        Needed!();
    };
}

macro_rules! State {
    () => {
        deps!();
        enum State < E > { Running , Done , Failure (E) , Incomplete (Needed) , }
    };
}

State!();