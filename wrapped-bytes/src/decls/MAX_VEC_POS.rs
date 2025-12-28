macro_rules! MAX_VEC_POS {
    () => {
        const MAX_VEC_POS : usize = usize :: MAX >> VEC_POS_OFFSET ;
    };
}

MAX_VEC_POS!();