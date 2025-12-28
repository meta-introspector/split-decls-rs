macro_rules! deps {
    () => {
        Outcome!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Outcome < '_ > { # [doc = " Return ourselves as (in-memory) bytes if possible."] pub fn as_bytes (& self) -> Option < usize > { match self { Outcome :: Written { bytes } => Some (* bytes) , Outcome :: Delayed { .. } => None , } } }
    };
}

impl_21!()