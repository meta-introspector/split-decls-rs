macro_rules! deps {
    () => {
        Error!();
        Service!();
        Response!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl < Request , S : Service < Request > + ? Sized > Service < Request > for Box < S > { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn call (& self , req : Request) -> Self :: Future { (* * self) . call (req) } }
    };
}

impl_196!();