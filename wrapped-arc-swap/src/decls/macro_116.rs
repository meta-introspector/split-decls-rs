macro_rules! macro_116 {
    () => {
        # [cfg (all (feature = "internal-test-strategies" , feature = "experimental-thread-local"))] compile_error ! ("experimental-thread-local is incompatible with internal-test-strategies as it enables #[no_std]") ;
    };
}

macro_116!()