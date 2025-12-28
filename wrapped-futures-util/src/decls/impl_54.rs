macro_rules! deps {
    () => {
        FnOnce1!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < Fut , F , T > FusedFuture for Map < Fut , F > where Fut : Future , F : FnOnce1 < Fut :: Output , Output = T > , { fn is_terminated (& self) -> bool { match self { Self :: Incomplete { .. } => false , Self :: Complete => true , } } }
    };
}

impl_54!();