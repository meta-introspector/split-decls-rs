macro_rules! Blake2b {
    () => {
        # [derive (Debug , Clone)] # [doc = " BLAKE2b streaming state."] pub struct Blake2b { _state : blake2b_core :: State , }
    };
}

Blake2b!();