macro_rules! deps {
    () => {
        StateRef!();
        State!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        # [doc = " Access"] impl StateRef < '_ > { # [doc = " Turn ourselves into our owned counterpart."] pub fn to_owned (self) -> State { self . into () } }
    };
}

impl_22!();