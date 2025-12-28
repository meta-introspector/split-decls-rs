macro_rules! deps {
    () => {
        Needed!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl Needed { # [doc = " Creates `Needed` instance, returns `Needed::Unknown` if the argument is zero"] pub fn new (s : usize) -> Self { match NonZeroUsize :: new (s) { Some (sz) => Needed :: Size (sz) , None => Needed :: Unknown , } } # [doc = " Indicates if we know how many bytes we need"] pub fn is_known (& self) -> bool { * self != Unknown } # [doc = " Maps a `Needed` to `Needed` by applying a function to a contained `Size` value."] # [inline] pub fn map < F : Fn (NonZeroUsize) -> usize > (self , f : F) -> Needed { match self { Unknown => Unknown , Size (n) => Needed :: new (f (n)) , } } }
    };
}

impl_146!()