macro_rules! CallbackFunc {
    () => {
        # [doc = " Callback function and user used in `compress_to_output`."] pub struct CallbackFunc < 'a > { pub put_buf_func : & 'a mut dyn FnMut (& [u8]) -> bool , }
    };
}

CallbackFunc!()