macro_rules! deps {
    () => {
        RawIterRange!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < T > Clone for RawIterRange < T > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Self { data : self . data . clone () , next_ctrl : self . next_ctrl , current_group : self . current_group . clone () , end : self . end , } } }
    };
}

impl_72!()