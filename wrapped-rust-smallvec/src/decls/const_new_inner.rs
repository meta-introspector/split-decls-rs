macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! const_new_inner {
    () => {
        deps!();
        const fn const_new_inner () -> SmallVec < i32 , 4 > { SmallVec :: < i32 , 4 > :: new () }
    };
}

const_new_inner!()