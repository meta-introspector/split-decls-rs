macro_rules! deps {
    () => {
        OptNonNullExt!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < T > OptNonNullExt < T > for Option < NonNull < T > > { # [inline] fn as_ptr (self) -> * mut T { match self { Some (ptr) => ptr . as_ptr () , None => ptr :: null_mut () , } } }
    };
}

impl_116!()