macro_rules! deps {
    () => {
        AssocItemLoc!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < N : AstIdNode > PartialEq for AssocItemLoc < N > { fn eq (& self , other : & Self) -> bool { self . container == other . container && self . id == other . id } }
    };
}

impl_34!()