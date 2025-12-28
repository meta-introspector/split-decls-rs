macro_rules! deps {
    () => {
        ExecuteFutFactory!();
        Data!();
        Extension!();
    };
}

macro_rules! NextExecute {
    () => {
        deps!();
        # [doc = " The remainder of a extension chain for execute."] pub struct NextExecute < 'a > { chain : & 'a [Arc < dyn Extension >] , execute_fut_factory : ExecuteFutFactory < 'a > , execute_data : Option < Data > , }
    };
}

NextExecute!()