macro_rules! deps {
    () => {
        Human!();
        Pet!();
        Object!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        # [Object (internal)] impl Human { async fn name (& self , surname : Option < bool >) -> Option < String > { unimplemented ! () } async fn pets (& self) -> Option < Vec < Option < Pet > > > { unimplemented ! () } async fn relatives (& self) -> Option < Vec < Human > > { unimplemented ! () } async fn iq (& self) -> Option < i32 > { unimplemented ! () } }
    };
}

impl_149!()