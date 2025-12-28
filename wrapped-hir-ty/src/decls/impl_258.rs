macro_rules! deps {
    () => {
        InferenceResult!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl < 'db > Index < PatId > for InferenceResult < 'db > { type Output = Ty < 'db > ; fn index (& self , pat : PatId) -> & Ty < 'db > { self . type_of_pat . get (pat) . unwrap_or (& self . error_ty) } }
    };
}

impl_258!();