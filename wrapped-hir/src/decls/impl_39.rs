macro_rules! deps {
    () => {
        Local!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl From < (DefWithBodyId , BindingId) > for Local { fn from ((parent , binding_id) : (DefWithBodyId , BindingId)) -> Self { Local { parent , binding_id } } }
    };
}

impl_39!()