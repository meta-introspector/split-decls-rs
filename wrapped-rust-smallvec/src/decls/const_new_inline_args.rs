macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! const_new_inline_args {
    () => {
        deps!();
        const fn const_new_inline_args () -> SmallVec < i32 , 2 > { crate :: smallvec_inline ! [1 , 4] }
    };
}

const_new_inline_args!();