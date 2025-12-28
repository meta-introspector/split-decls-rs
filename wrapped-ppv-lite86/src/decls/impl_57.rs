macro_rules! deps {
    () => {
        Vec4Ext!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < W : Copy > Vec4Ext < W > for x4 < W > { # [inline (always)] fn transpose4 (a : Self , b : Self , c : Self , d : Self) -> (Self , Self , Self , Self) where Self : Sized , { (x4 ([a . 0 [0] , b . 0 [0] , c . 0 [0] , d . 0 [0]]) , x4 ([a . 0 [1] , b . 0 [1] , c . 0 [1] , d . 0 [1]]) , x4 ([a . 0 [2] , b . 0 [2] , c . 0 [2] , d . 0 [2]]) , x4 ([a . 0 [3] , b . 0 [3] , c . 0 [3] , d . 0 [3]]) ,) } }
    };
}

impl_57!();