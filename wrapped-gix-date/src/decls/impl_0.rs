macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! impl_0 {
    () => {
        deps!();
        # [doc = " Access"] impl Time { # [doc = " Return true if this time has been initialized to anything non-default, i.e. 0."] pub fn is_set (& self) -> bool { * self != Self :: default () } }
    };
}

impl_0!();