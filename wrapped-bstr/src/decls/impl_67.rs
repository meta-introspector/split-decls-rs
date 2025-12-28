macro_rules! deps {
    () => {
        Finder!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < 'a > Finder < 'a > { # [doc = " Create a new finder for the given needle."] # [inline] pub fn new < B : ? Sized + AsRef < [u8] > > (needle : & 'a B) -> Finder < 'a > { Finder (memmem :: Finder :: new (needle . as_ref ())) } # [doc = " Convert this finder into its owned variant, such that it no longer"] # [doc = " borrows the needle."] # [doc = ""] # [doc = " If this is already an owned finder, then this is a no-op. Otherwise,"] # [doc = " this copies the needle."] # [doc = ""] # [doc = " This is only available when the `alloc` feature is enabled."] # [cfg (feature = "alloc")] # [inline] pub fn into_owned (self) -> Finder < 'static > { Finder (self . 0 . into_owned ()) } # [doc = " Returns the needle that this finder searches for."] # [doc = ""] # [doc = " Note that the lifetime of the needle returned is tied to the lifetime"] # [doc = " of the finder, and may be shorter than the `'a` lifetime. Namely, a"] # [doc = " finder's needle can be either borrowed or owned, so the lifetime of the"] # [doc = " needle returned must necessarily be the shorter of the two."] # [inline] pub fn needle (& self) -> & [u8] { self . 0 . needle () } # [doc = " Returns the index of the first occurrence of this needle in the given"] # [doc = " haystack."] # [doc = ""] # [doc = " The haystack may be any type that can be cheaply converted into a"] # [doc = " `&[u8]`. This includes, but is not limited to, `&str` and `&[u8]`."] # [doc = ""] # [doc = " # Complexity"] # [doc = ""] # [doc = " This routine is guaranteed to have worst case linear time complexity"] # [doc = " with respect to both the needle and the haystack. That is, this runs"] # [doc = " in `O(needle.len() + haystack.len())` time."] # [doc = ""] # [doc = " This routine is also guaranteed to have worst case constant space"] # [doc = " complexity."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::Finder;"] # [doc = ""] # [doc = " let haystack = \"foo bar baz\";"] # [doc = " assert_eq!(Some(0), Finder::new(\"foo\").find(haystack));"] # [doc = " assert_eq!(Some(4), Finder::new(\"bar\").find(haystack));"] # [doc = " assert_eq!(None, Finder::new(\"quux\").find(haystack));"] # [doc = " ```"] # [inline] pub fn find < B : AsRef < [u8] > > (& self , haystack : B) -> Option < usize > { self . 0 . find (haystack . as_ref ()) } }
    };
}

impl_67!()