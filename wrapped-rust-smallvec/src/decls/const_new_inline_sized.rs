macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! const_new_inline_sized {
    () => {
        deps!();
        const fn const_new_inline_sized () -> SmallVec < i32 , 4 > { crate :: smallvec_inline ! [1 ; 4] }
    };
}

const_new_inline_sized!();