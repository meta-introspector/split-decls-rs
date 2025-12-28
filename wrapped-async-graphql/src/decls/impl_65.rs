macro_rules! deps {
    () => {
        Guard!();
        And!();
        Context!();
        Result!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < A : Guard + Send + Sync , B : Guard + Send + Sync > Guard for And < A , B > { async fn check (& self , ctx : & Context < '_ >) -> Result < () > { self . 0 . check (ctx) . await ? ; self . 1 . check (ctx) . await } }
    };
}

impl_65!()