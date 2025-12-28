macro_rules! deps {
    () => {
        Shake!();
    };
}

macro_rules! Shake256 {
    () => {
        deps!();
        # [derive (Clone , Debug)] # [doc = " SHAKE-256 streaming state."] pub struct Shake256 { pub (crate) _state : Shake < SHAKE_256_RATE > , }
    };
}

Shake256!();