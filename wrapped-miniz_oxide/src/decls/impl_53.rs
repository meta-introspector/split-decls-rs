macro_rules! deps {
    () => {
        CallbackFunc!();
        TDEFLStatus!();
        ParamsOxide!();
        SavedOutputBufferOxide!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl CallbackFunc < '_ > { fn flush_output (& mut self , saved_output : SavedOutputBufferOxide , params : & mut ParamsOxide ,) -> i32 { let call_success = (self . put_buf_func) (& params . local_buf . b [0 .. saved_output . pos]) ; if ! call_success { params . prev_return_status = TDEFLStatus :: PutBufFailed ; return params . prev_return_status as i32 ; } params . flush_remaining as i32 } }
    };
}

impl_53!();