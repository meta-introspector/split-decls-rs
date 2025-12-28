macro_rules! N_ABS {
    () => {
        # [doc = " An absolute symbol. The symbol has a value but is not relocatable."] pub const N_ABS : i16 = - 1 ;
    };
}

N_ABS!();