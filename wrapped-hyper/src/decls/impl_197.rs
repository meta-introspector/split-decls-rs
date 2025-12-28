macro_rules! deps {
    () => {
        Response!();
        Service!();
        Error!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < Request , S : Service < Request > + ? Sized > Service < Request > for std :: rc :: Rc < S > { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn call (& self , req : Request) -> Self :: Future { (* * self) . call (req) } }
    };
}

impl_197!();