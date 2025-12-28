macro_rules! deps {
    () => {
        Result!();
        ServiceFn!();
        Response!();
        Service!();
        Error!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl < F , ReqBody , Ret , ResBody , E > Service < Request < ReqBody > > for ServiceFn < F , ReqBody > where F : Fn (Request < ReqBody >) -> Ret , ReqBody : Body , Ret : Future < Output = Result < Response < ResBody > , E > > , E : Into < Box < dyn StdError + Send + Sync > > , ResBody : Body , { type Response = crate :: Response < ResBody > ; type Error = E ; type Future = Ret ; fn call (& self , req : Request < ReqBody >) -> Self :: Future { (self . f) (req) } }
    };
}

impl_202!();