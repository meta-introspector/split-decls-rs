macro_rules! deps {
    () => {
        Elem!();
    };
}

macro_rules! AddToSllResult {
    () => {
        deps!();
        pub (crate) enum AddToSllResult < 'a , E : Elem > { NoHead , EmptyHead (& 'a Cell < * const E >) , SmallerThanHead (& 'a Cell < * const E >) , SmallerThanNotHead (* const E) , AlreadyInSll (* const E) , }
    };
}

AddToSllResult!()