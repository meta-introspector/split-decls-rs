macro_rules! deps {
    () => {
        Execution!();
    };
}

macro_rules! fence_rel {
    () => {
        deps!();
        fn fence_rel (execution : & mut Execution) { let active = execution . threads . active_mut () ; active . released = active . causality ; }
    };
}

fence_rel!()