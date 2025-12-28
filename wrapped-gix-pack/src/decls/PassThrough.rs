macro_rules! PassThrough {
    () => {
        struct PassThrough < R , W > { read : R , write : W , }
    };
}

PassThrough!();