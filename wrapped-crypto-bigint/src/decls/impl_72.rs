macro_rules! deps {
    () => {
        ConstCtOption!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < MOD : ConstMontyParams < LIMBS > , const LIMBS : usize > ConstCtOption < ConstMontyForm < MOD , LIMBS > > { # [doc = " This returns the underlying value if it is `Some` or the provided value otherwise."] # [inline] pub const fn unwrap_or (self , def : ConstMontyForm < MOD , LIMBS >) -> ConstMontyForm < MOD , LIMBS > { ConstMontyForm :: < MOD , LIMBS > :: select (& def , & self . value , self . is_some) } # [doc = " Returns the contained value, consuming the `self` value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the value is none with a custom panic message provided by `msg`."] # [inline] # [track_caller] pub const fn expect (self , msg : & str) -> ConstMontyForm < MOD , LIMBS > { assert ! (self . is_some . is_true_vartime () , "{}" , msg) ; self . value } }
    };
}

impl_72!();