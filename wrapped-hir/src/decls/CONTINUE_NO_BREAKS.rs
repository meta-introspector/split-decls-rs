macro_rules! CONTINUE_NO_BREAKS {
    () => {
        const CONTINUE_NO_BREAKS : ControlFlow < Infallible , () > = ControlFlow :: Continue (()) ;
    };
}

CONTINUE_NO_BREAKS!();