macro_rules! deps {
    () => {
        Decoder!();
        DecodeError!();
    };
}

macro_rules! impl_406 {
    () => {
        deps!();
        impl < T > Decoder for & mut T where T : Decoder , { type R = T :: R ; type C = T :: C ; type Context = T :: Context ; fn reader (& mut self) -> & mut Self :: R { T :: reader (self) } fn config (& self) -> & Self :: C { T :: config (self) } # [inline] fn claim_bytes_read (& mut self , n : usize) -> Result < () , DecodeError > { T :: claim_bytes_read (self , n) } # [inline] fn unclaim_bytes_read (& mut self , n : usize) { T :: unclaim_bytes_read (self , n) } fn context (& mut self) -> & mut Self :: Context { T :: context (self) } }
    };
}

impl_406!()