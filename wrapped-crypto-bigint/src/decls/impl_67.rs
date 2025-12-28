macro_rules! deps {
    () => {
        ConstCtOption!();
        Int!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < const LIMBS : usize > ConstCtOption < Int < LIMBS > > { # [doc = " This returns the underlying value if it is `Some` or the provided value otherwise."] # [inline] pub const fn unwrap_or (self , def : Int < LIMBS >) -> Int < LIMBS > { Int :: select (& def , & self . value , self . is_some) } # [doc = " Returns the contained value, consuming the `self` value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the value is none with a custom panic message provided by"] # [doc = " `msg`."] # [inline] # [track_caller] pub const fn expect (self , msg : & str) -> Int < LIMBS > { assert ! (self . is_some . is_true_vartime () , "{}" , msg) ; self . value } }
    };
}

impl_67!();