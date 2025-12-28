macro_rules! CallbackBuf {
    () => {
        struct CallbackBuf < 'a > { pub out_buf : & 'a mut [u8] , }
    };
}

CallbackBuf!();