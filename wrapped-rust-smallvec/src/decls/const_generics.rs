macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! const_generics {
    () => {
        deps!();
        # [test] fn const_generics () { let _v = SmallVec :: < i32 , 987 > :: default () ; }
    };
}

const_generics!();