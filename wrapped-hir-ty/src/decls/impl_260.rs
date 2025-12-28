macro_rules! deps {
    () => {
        InferenceResult!();
    };
}

macro_rules! impl_260 {
    () => {
        deps!();
        impl < 'db > Index < BindingId > for InferenceResult < 'db > { type Output = Ty < 'db > ; fn index (& self , b : BindingId) -> & Ty < 'db > { self . type_of_binding . get (b) . unwrap_or (& self . error_ty) } }
    };
}

impl_260!();