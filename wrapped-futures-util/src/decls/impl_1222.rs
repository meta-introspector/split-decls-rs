macro_rules! deps {
    () => {
        Window!();
    };
}

macro_rules! impl_1222 {
    () => {
        deps!();
        impl < T : AsRef < [u8] > > Window < T > { # [doc = " Creates a new window around the buffer `t` defaulting to the entire"] # [doc = " slice."] # [doc = ""] # [doc = " Further methods can be called on the returned `Window<T>` to alter the"] # [doc = " window into the data provided."] pub fn new (t : T) -> Self { Self { range : 0 .. t . as_ref () . len () , inner : t } } # [doc = " Gets a shared reference to the underlying buffer inside of this"] # [doc = " `Window`."] pub fn get_ref (& self) -> & T { & self . inner } # [doc = " Gets a mutable reference to the underlying buffer inside of this"] # [doc = " `Window`."] pub fn get_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Consumes this `Window`, returning the underlying buffer."] pub fn into_inner (self) -> T { self . inner } # [doc = " Returns the starting index of this window into the underlying buffer"] # [doc = " `T`."] pub fn start (& self) -> usize { self . range . start } # [doc = " Returns the end index of this window into the underlying buffer"] # [doc = " `T`."] pub fn end (& self) -> usize { self . range . end } # [doc = " Changes the range of this window to the range specified."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This method will panic if `range` is out of bounds for the underlying"] # [doc = " slice or if [`start_bound()`] of `range` comes after the [`end_bound()`]."] # [doc = ""] # [doc = " [`start_bound()`]: std::ops::RangeBounds::start_bound"] # [doc = " [`end_bound()`]: std::ops::RangeBounds::end_bound"] pub fn set < R : RangeBounds < usize > > (& mut self , range : R) { let start = match range . start_bound () { Bound :: Included (n) => * n , Bound :: Excluded (n) => * n + 1 , Bound :: Unbounded => 0 , } ; let end = match range . end_bound () { Bound :: Included (n) => * n + 1 , Bound :: Excluded (n) => * n , Bound :: Unbounded => self . inner . as_ref () . len () , } ; assert ! (end <= self . inner . as_ref () . len ()) ; assert ! (start <= end) ; self . range . start = start ; self . range . end = end ; } }
    };
}

impl_1222!()