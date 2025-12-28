macro_rules! deps {
    () => {
        ParamsOxide!();
        CallbackBuf!();
        SavedOutputBufferOxide!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl CallbackBuf < '_ > { fn flush_output (& mut self , saved_output : SavedOutputBufferOxide , params : & mut ParamsOxide ,) -> i32 { if saved_output . local { let n = cmp :: min (saved_output . pos , self . out_buf . len () - params . out_buf_ofs) ; (self . out_buf [params . out_buf_ofs .. params . out_buf_ofs + n]) . copy_from_slice (& params . local_buf . b [.. n]) ; params . out_buf_ofs += n ; if saved_output . pos != n { params . flush_ofs = n as u32 ; params . flush_remaining = (saved_output . pos - n) as u32 ; } } else { params . out_buf_ofs += saved_output . pos ; } params . flush_remaining as i32 } }
    };
}

impl_55!()