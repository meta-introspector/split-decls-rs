macro_rules! deps {
    () => {
        IvSizeUser!();
        Iv!();
    };
}

macro_rules! IvState {
    () => {
        deps!();
        # [doc = " Trait for loading current IV state."] pub trait IvState : IvSizeUser { # [doc = " Returns current IV state."] fn iv_state (& self) -> Iv < Self > ; }
    };
}

IvState!()