macro_rules! deps {
    () => {
        WithContext!();
        DecodeError!();
        Decoder!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl < Context , D : Decoder + ? Sized > Decoder for WithContext < '_ , D , Context > { type R = D :: R ; type C = D :: C ; type Context = Context ; fn context (& mut self) -> & mut Self :: Context { & mut self . context } fn reader (& mut self) -> & mut Self :: R { self . decoder . reader () } fn config (& self) -> & Self :: C { self . decoder . config () } fn claim_bytes_read (& mut self , n : usize) -> Result < () , DecodeError > { self . decoder . claim_bytes_read (n) } fn unclaim_bytes_read (& mut self , n : usize) { self . decoder . unclaim_bytes_read (n) } }
    };
}

impl_279!();