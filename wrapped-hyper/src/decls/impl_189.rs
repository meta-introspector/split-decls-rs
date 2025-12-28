macro_rules! deps {
    () => {
        HttpService!();
        Error!();
        Response!();
        Service!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < T , B1 , B2 > HttpService < B1 > for T where T : Service < Request < B1 > , Response = Response < B2 > > , B2 : Body , T :: Error : Into < Box < dyn StdError + Send + Sync > > , { type ResBody = B2 ; type Error = T :: Error ; type Future = T :: Future ; fn call (& mut self , req : Request < B1 >) -> Self :: Future { Service :: call (self , req) } }
    };
}

impl_189!();