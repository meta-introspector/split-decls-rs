macro_rules! deps {
    () => {
        Field!();
        Context!();
        Result!();
    };
}

macro_rules! Guard {
    () => {
        deps!();
        # [doc = " Field guard"] # [doc = ""] # [doc = " Guard is a pre-condition for a field that is resolved if `Ok(())` is"] # [doc = " returned, otherwise an error is returned."] # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] pub trait Guard { # [doc = " Check whether the guard will allow access to the field."] # [cfg (feature = "boxed-trait")] async fn check (& self , ctx : & Context < '_ >) -> Result < () > ; # [doc = " Check whether the guard will allow access to the field."] # [cfg (not (feature = "boxed-trait"))] fn check (& self , ctx : & Context < '_ >) -> impl Future < Output = Result < () > > + Send ; }
    };
}

Guard!()