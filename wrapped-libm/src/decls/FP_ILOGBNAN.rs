macro_rules! FP_ILOGBNAN {
    () => {
        const FP_ILOGBNAN : i32 = - 1 - 0x7fffffff ;
    };
}

FP_ILOGBNAN!()