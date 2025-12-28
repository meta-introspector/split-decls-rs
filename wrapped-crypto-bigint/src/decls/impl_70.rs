macro_rules! deps {
    () => {
        NonZero!();
        Limb!();
        ConstCtOption!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl ConstCtOption < NonZero < Limb > > { # [doc = " Returns the contained value, consuming the `self` value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the value is none with a custom panic message provided by"] # [doc = " `msg`."] # [inline] # [track_caller] pub const fn expect (self , msg : & str) -> NonZero < Limb > { assert ! (self . is_some . is_true_vartime () , "{}" , msg) ; self . value } }
    };
}

impl_70!()