macro_rules! deps {
    () => {
        HeapVec!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl PartialEq for HeapVec { # [inline] # [allow (clippy :: op_ref)] fn eq (& self , other : & Self) -> bool { use core :: ops :: Deref ; self . len () == other . len () && self . deref () == other . deref () } }
    };
}

impl_62!()