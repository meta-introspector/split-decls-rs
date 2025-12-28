macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl < T : ? Sized > Drop for Arc < T > { # [track_caller] fn drop (& mut self) { if self . obj . ref_dec (location ! ()) { assert_eq ! (1 , std :: sync :: Arc :: strong_count (& self . value) , "something odd is going on") ; self . unregister () ; } } }
    };
}

impl_243!();