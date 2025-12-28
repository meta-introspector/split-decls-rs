macro_rules! deps {
    () => {
        Alien!();
        Object!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        # [Object (internal)] impl Alien { async fn name (& self , surname : Option < bool >) -> Option < String > { unimplemented ! () } async fn iq (& self) -> Option < i32 > { unimplemented ! () } async fn num_eyes (& self) -> Option < i32 > { unimplemented ! () } }
    };
}

impl_151!();