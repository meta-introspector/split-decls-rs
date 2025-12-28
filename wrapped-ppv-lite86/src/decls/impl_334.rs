macro_rules! deps {
    () => {
        Vector!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl Vector < [u32 ; 16] > for u32x4x4_generic { fn to_scalars (self) -> [u32 ; 16] { let [a , b , c , d] = self . 0 ; let a = a . 0 ; let b = b . 0 ; let c = c . 0 ; let d = d . 0 ; [a [0] , a [1] , a [2] , a [3] , b [0] , b [1] , b [2] , b [3] , c [0] , c [1] , c [2] , c [3] , d [0] , d [1] , d [2] , d [3] ,] } }
    };
}

impl_334!();