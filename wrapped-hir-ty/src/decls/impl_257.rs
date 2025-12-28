macro_rules! deps {
    () => {
        InferenceResult!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl < 'db > Index < ExprId > for InferenceResult < 'db > { type Output = Ty < 'db > ; fn index (& self , expr : ExprId) -> & Ty < 'db > { self . type_of_expr . get (expr) . unwrap_or (& self . error_ty) } }
    };
}

impl_257!()