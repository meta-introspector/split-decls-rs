macro_rules! deps {
    () => {
        TDEFLStatus!();
        CallbackFunc!();
        CallbackOxide!();
        CompressorOxide!();
        TDEFLFlush!();
    };
}

macro_rules! compress_to_output {
    () => {
        deps!();
        # [doc = " Main compression function. Callbacks output."] # [doc = ""] # [doc = " # Returns"] # [doc = " Returns a tuple containing the current status of the compressor, the current position"] # [doc = " in the input buffer."] # [doc = ""] # [doc = " The caller is responsible for ensuring the `CallbackFunc` struct will not cause undefined"] # [doc = " behaviour."] pub fn compress_to_output (d : & mut CompressorOxide , in_buf : & [u8] , flush : TDEFLFlush , mut callback_func : impl FnMut (& [u8]) -> bool ,) -> (TDEFLStatus , usize) { let res = compress_inner (d , & mut CallbackOxide :: new_callback_func (in_buf , CallbackFunc { put_buf_func : & mut callback_func , } ,) , flush ,) ; (res . 0 , res . 1) }
    };
}

compress_to_output!()