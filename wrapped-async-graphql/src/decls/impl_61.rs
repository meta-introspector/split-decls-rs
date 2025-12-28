macro_rules! deps {
    () => {
        Context!();
        Guard!();
        Result!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T > Guard for T where T : Fn (& Context < '_ >) -> Result < () > + Send + Sync + 'static , { async fn check (& self , ctx : & Context < '_ >) -> Result < () > { self (ctx) } }
    };
}

impl_61!();