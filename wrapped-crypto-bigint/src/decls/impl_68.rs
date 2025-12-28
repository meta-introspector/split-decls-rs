macro_rules! deps {
    () => {
        NonZeroInt!();
        ConstCtOption!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < const LIMBS : usize > ConstCtOption < NonZeroInt < LIMBS > > { # [doc = " Returns the contained value, consuming the `self` value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the value is none with a custom panic message provided by"] # [doc = " `msg`."] # [inline] # [track_caller] pub const fn expect (self , msg : & str) -> NonZeroInt < LIMBS > { assert ! (self . is_some . is_true_vartime () , "{}" , msg) ; self . value } }
    };
}

impl_68!()