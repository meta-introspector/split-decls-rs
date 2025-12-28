macro_rules! deps {
    () => {
        Finder!();
        Mask!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl Finder { const MAX_NEEDLE_LEN : usize = (Mask :: BITS - 1) as usize ; # [doc = " Create a new Shift-Or forward searcher for the given `needle`."] # [doc = ""] # [doc = " The needle may be empty. The empty needle matches at every byte offset."] # [inline] pub fn new (needle : & [u8]) -> Option < Finder > { let needle_len = needle . len () ; if needle_len > Finder :: MAX_NEEDLE_LEN { return None ; } let mut searcher = Finder { masks : Box :: from ([! 0 ; 256]) , needle_len } ; for (i , & byte) in needle . iter () . enumerate () { searcher . masks [usize :: from (byte)] &= ! (1 << i) ; } Some (searcher) } # [doc = " Return the first occurrence of the needle given to `Finder::new` in"] # [doc = " the `haystack` given. If no such occurrence exists, then `None` is"] # [doc = " returned."] # [doc = ""] # [doc = " Unlike most other substring search implementations in this crate, this"] # [doc = " finder does not require passing the needle at search time. A match can"] # [doc = " be determined without the needle at all since the required information"] # [doc = " is already encoded into this finder at construction time."] # [doc = ""] # [doc = " The maximum value this can return is `haystack.len()`, which can only"] # [doc = " occur when the needle and haystack both have length zero. Otherwise,"] # [doc = " for non-empty haystacks, the maximum value is `haystack.len() - 1`."] # [inline] pub fn find (& self , haystack : & [u8]) -> Option < usize > { if self . needle_len == 0 { return Some (0) ; } let mut result = ! 1 ; for (i , & byte) in haystack . iter () . enumerate () { result |= self . masks [usize :: from (byte)] ; result <<= 1 ; if result & (1 << self . needle_len) == 0 { return Some (i + 1 - self . needle_len) ; } } None } }
    };
}

impl_101!();