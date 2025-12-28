macro_rules! deps {
    () => {
        ConstCtOption!();
        ConstChoice!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < T > ConstCtOption < T > { # [inline] pub (crate) const fn new (value : T , is_some : ConstChoice) -> Self { Self { value , is_some } } # [inline] pub (crate) const fn some (value : T) -> Self { Self { value , is_some : ConstChoice :: TRUE , } } # [inline] pub (crate) const fn none (dummy_value : T) -> Self { Self { value : dummy_value , is_some : ConstChoice :: FALSE , } } # [doc = " Returns a reference to the contents of this structure."] # [doc = ""] # [doc = " **Note:** if the second element is `None`, the first value may take any value."] # [inline] pub (crate) const fn components_ref (& self) -> (& T , ConstChoice) { (& self . value , self . is_some) } # [doc = " Returns a true [`ConstChoice`] if this value is `Some`."] # [inline] pub const fn is_some (& self) -> ConstChoice { self . is_some } # [doc = " Returns a true [`ConstChoice`] if this value is `None`."] # [inline] pub const fn is_none (& self) -> ConstChoice { self . is_some . not () } # [doc = " This returns the underlying value but panics if it is not `Some`."] # [inline] # [track_caller] pub fn unwrap (self) -> T { assert ! (self . is_some . is_true_vartime () , "called `ConstCtOption::unwrap()` on a `None` value") ; self . value } # [doc = " Apply an additional [`ConstChoice`] requirement to `is_some`."] # [inline] pub (crate) const fn and_choice (mut self , is_some : ConstChoice) -> Self { self . is_some = self . is_some . and (is_some) ; self } }
    };
}

impl_60!();