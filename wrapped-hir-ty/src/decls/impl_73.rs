macro_rules! deps {
    () => {
        InferenceResult!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < 'db > Index < ExprOrPatId > for InferenceResult < 'db > { type Output = Ty < 'db > ; fn index (& self , id : ExprOrPatId) -> & Ty < 'db > { match id { ExprOrPatId :: ExprId (id) => & self [id] , ExprOrPatId :: PatId (id) => & self [id] , } } }
    };
}

impl_73!();