macro_rules! deps {
    () => {
        ErrorExtensionValues!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl ErrorExtensionValues { # [doc = " Set an extension value."] pub fn set (& mut self , name : impl AsRef < str > , value : impl Into < Value >) { self . 0 . insert (name . as_ref () . to_string () , value . into ()) ; } # [doc = " Unset an extension value."] pub fn unset (& mut self , name : impl AsRef < str >) { self . 0 . remove (name . as_ref ()) ; } # [doc = " Get an extension value."] pub fn get (& self , name : impl AsRef < str >) -> Option < & Value > { self . 0 . get (name . as_ref ()) } }
    };
}

impl_26!();