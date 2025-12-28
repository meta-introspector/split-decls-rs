macro_rules! dead_code_ice_workaround {
    () => {
        # [expect (dead_code)] fn dead_code_ice_workaround () { }
    };
}

dead_code_ice_workaround!();