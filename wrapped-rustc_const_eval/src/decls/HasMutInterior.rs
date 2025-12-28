macro_rules! HasMutInterior {
    () => {
        # [doc = " Constant containing interior mutability (`UnsafeCell<T>`)."] # [doc = " This must be ruled out to make sure that evaluating the constant at compile-time"] # [doc = " and at *any point* during the run-time would produce the same result. In particular,"] # [doc = " promotion of temporaries must not change program behavior; if the promoted could be"] # [doc = " written to, that would be a problem."] pub struct HasMutInterior ;
    };
}

HasMutInterior!()