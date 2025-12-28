macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl < T > Arc < T > { # [doc = " Constructs a new `Arc<T>`."] # [track_caller] pub fn new (value : T) -> Arc < T > { let std = std :: sync :: Arc :: new (value) ; Arc :: from_std (std) } # [doc = " Constructs a new `Pin<Arc<T>>`."] pub fn pin (data : T) -> Pin < Arc < T > > { unsafe { Pin :: new_unchecked (Arc :: new (data)) } } # [doc = " Returns the inner value, if the `Arc` has exactly one strong reference."] # [track_caller] pub fn try_unwrap (this : Arc < T >) -> Result < T , Arc < T > > { if ! this . obj . get_mut (location ! ()) { return Err (this) ; } assert_eq ! (1 , std :: sync :: Arc :: strong_count (& this . value)) ; this . obj . ref_dec (location ! ()) ; this . unregister () ; let arc_value = unsafe { let _arc_obj = ptr :: read (& this . obj) ; let arc_value = ptr :: read (& this . value) ; mem :: forget (this) ; arc_value } ; match std :: sync :: Arc :: try_unwrap (arc_value) { Ok (value) => Ok (value) , Err (_) => unreachable ! () , } } }
    };
}

impl_239!();