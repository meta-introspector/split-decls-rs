macro_rules! deps {
    () => {
        Error!();
        Response!();
        Service!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < Request , S : Service < Request > + ? Sized > Service < Request > for & '_ S { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn call (& self , req : Request) -> Self :: Future { (* * self) . call (req) } }
    };
}

impl_194!()