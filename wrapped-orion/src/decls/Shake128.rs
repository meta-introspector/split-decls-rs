macro_rules! deps {
    () => {
        Shake!();
    };
}

macro_rules! Shake128 {
    () => {
        deps!();
        # [derive (Clone , Debug)] # [doc = " SHAKE-128 streaming state."] pub struct Shake128 { pub (crate) _state : Shake < SHAKE_128_RATE > , }
    };
}

Shake128!();