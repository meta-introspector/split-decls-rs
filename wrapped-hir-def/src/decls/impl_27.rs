macro_rules! deps {
    () => {
        ItemLoc!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < N : AstIdNode > PartialEq for ItemLoc < N > { fn eq (& self , other : & Self) -> bool { self . container == other . container && self . id == other . id } }
    };
}

impl_27!()