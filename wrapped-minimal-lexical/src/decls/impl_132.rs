macro_rules! deps {
    () => {
        StackVec!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl PartialEq for StackVec { # [inline] # [allow (clippy :: op_ref)] fn eq (& self , other : & Self) -> bool { use core :: ops :: Deref ; self . len () == other . len () && self . deref () == other . deref () } }
    };
}

impl_132!();