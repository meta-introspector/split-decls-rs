macro_rules! deps {
    () => {
        FluentArgs!();
        FluentValue!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < 'args > FluentArgs < 'args > { # [doc = " Creates a new empty argument map."] pub fn new () -> Self { Self :: default () } # [doc = " Pre-allocates capacity for arguments."] pub fn with_capacity (capacity : usize) -> Self { Self (Vec :: with_capacity (capacity)) } # [doc = " Gets the [`FluentValue`] at the `key` if it exists."] pub fn get < K > (& self , key : K) -> Option < & FluentValue < 'args > > where K : Into < Cow < 'args , str > > , { let key = key . into () ; if let Ok (idx) = self . 0 . binary_search_by_key (& & key , | (k , _) | k) { Some (& self . 0 [idx] . 1) } else { None } } # [doc = " Sets the key value pair."] pub fn set < K , V > (& mut self , key : K , value : V) where K : Into < Cow < 'args , str > > , V : Into < FluentValue < 'args > > , { let key = key . into () ; match self . 0 . binary_search_by_key (& & key , | (k , _) | k) { Ok (idx) => self . 0 [idx] = (key , value . into ()) , Err (idx) => self . 0 . insert (idx , (key , value . into ())) , } ; } # [doc = " Iterate over a tuple of the key an [`FluentValue`]."] pub fn iter (& self) -> impl Iterator < Item = (& str , & FluentValue < '_ >) > { self . 0 . iter () . map (| (k , v) | (k . as_ref () , v)) } }
    };
}

impl_1!();