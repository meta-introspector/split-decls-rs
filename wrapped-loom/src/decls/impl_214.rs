macro_rules! deps {
    () => {
        UnsafeCell!();
        Cell!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl < T > UnsafeCell < T > { # [doc = " Constructs a new instance of `UnsafeCell` which will wrap the specified value."] # [track_caller] pub fn new (data : T) -> UnsafeCell < T > { let state = rt :: Cell :: new (location ! ()) ; UnsafeCell { state , data : std :: cell :: UnsafeCell :: new (data) , } } # [doc = " Unwraps the value."] pub fn into_inner (self) -> T { self . data . into_inner () } }
    };
}

impl_214!()