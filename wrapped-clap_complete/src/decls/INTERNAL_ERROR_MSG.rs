macro_rules! INTERNAL_ERROR_MSG {
    () => {
        const INTERNAL_ERROR_MSG : & str = "Fatal internal error. Please consider filing a bug \
                                  report at https://github.com/clap-rs/clap/issues" ;
    };
}

INTERNAL_ERROR_MSG!()