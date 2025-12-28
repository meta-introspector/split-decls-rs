macro_rules! deps {
    () => {
        DecoderImpl!();
        Decoder!();
        Reader!();
        DecodeError!();
        Config!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < R : Reader , C : Config , Context > Decoder for DecoderImpl < R , C , Context > { type R = R ; type C = C ; type Context = Context ; fn reader (& mut self) -> & mut Self :: R { & mut self . reader } fn config (& self) -> & Self :: C { & self . config } # [inline] fn claim_bytes_read (& mut self , n : usize) -> Result < () , DecodeError > { if let Some (limit) = C :: LIMIT { self . bytes_read = self . bytes_read . checked_add (n) . ok_or (DecodeError :: LimitExceeded) ? ; if self . bytes_read > limit { Err (DecodeError :: LimitExceeded) } else { Ok (()) } } else { Ok (()) } } # [inline] fn unclaim_bytes_read (& mut self , n : usize) { if C :: LIMIT . is_some () { self . bytes_read -= n ; } } fn context (& mut self) -> & mut Self :: Context { & mut self . context } }
    };
}

impl_276!();