macro_rules! deps {
    () => {
        Service!();
        Response!();
        Error!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl < Request , S : Service < Request > + ? Sized > Service < Request > for & '_ mut S { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn call (& self , req : Request) -> Self :: Future { (* * self) . call (req) } }
    };
}

impl_195!();