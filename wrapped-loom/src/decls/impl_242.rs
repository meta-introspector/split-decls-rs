macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl < T : ? Sized > Clone for Arc < T > { # [track_caller] fn clone (& self) -> Arc < T > { self . obj . ref_inc (location ! ()) ; Arc { value : self . value . clone () , obj : self . obj . clone () , } } }
    };
}

impl_242!();