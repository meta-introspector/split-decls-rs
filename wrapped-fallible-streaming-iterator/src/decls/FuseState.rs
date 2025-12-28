macro_rules! FuseState {
    () => {
        # [derive (Copy , Clone)] enum FuseState { Start , Middle , End , }
    };
}

FuseState!()