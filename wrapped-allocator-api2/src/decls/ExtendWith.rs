macro_rules! ExtendWith {
    () => {
        trait ExtendWith < T > { fn next (& mut self) -> T ; fn last (self) -> T ; }
    };
}

ExtendWith!();