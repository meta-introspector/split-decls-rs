macro_rules! deps {
    () => {
        Allocation!();
        Track!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl < T > Track < T > { # [doc = " Track a value for leaks"] # [track_caller] pub fn new (value : T) -> Track < T > { Track { value , _obj : rt :: Allocation :: new (location ! ()) , } } # [doc = " Get a reference to the value"] pub fn get_ref (& self) -> & T { & self . value } # [doc = " Get a mutable reference to the value"] pub fn get_mut (& mut self) -> & mut T { & mut self . value } # [doc = " Stop tracking the value for leaks"] pub fn into_inner (self) -> T { self . value } }
    };
}

impl_199!();