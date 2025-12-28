macro_rules! deps {
    () => {
        CallbackOut!();
    };
}

macro_rules! CallbackOxide {
    () => {
        deps!();
        pub (crate) struct CallbackOxide < 'a > { in_buf : Option < & 'a [u8] > , in_buf_size : Option < & 'a mut usize > , out_buf_size : Option < & 'a mut usize > , out : CallbackOut < 'a > , }
    };
}

CallbackOxide!();