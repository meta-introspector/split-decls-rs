macro_rules! Token {
    () => {
        # [derive (Clone)] # [allow (dead_code)] pub enum Token { V1 { nanos_since_1970 : u64 } , V2 { token : BString } , }
    };
}

Token!()