macro_rules! deps {
    () => {
        Response!();
        Data!();
    };
}

macro_rules! ExecuteFutFactory {
    () => {
        deps!();
        type ExecuteFutFactory < 'a > = Box < dyn FnOnce (Option < Data >) -> BoxFuture < 'a , Response > + Send + 'a > ;
    };
}

ExecuteFutFactory!()