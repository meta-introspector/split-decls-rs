macro_rules! deps {
    () => {
        Response!();
        Error!();
        Service!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl < Request , S : Service < Request > + ? Sized > Service < Request > for std :: sync :: Arc < S > { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn call (& self , req : Request) -> Self :: Future { (* * self) . call (req) } }
    };
}

impl_198!();