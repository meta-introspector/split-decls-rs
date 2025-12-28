macro_rules! deps {
    () => {
        Flags!();
        Iter!();
        BitFlags!();
        IterNames!();
    };
}

macro_rules! __impl_public_bitflags_iter {
    () => {
        deps!();
        # [doc = " Implement iterators on the public (user-facing) bitflags type."] # [macro_export] # [doc (hidden)] macro_rules ! __impl_public_bitflags_iter { ($ (# [$ outer : meta]) * $ BitFlags : ident : $ T : ty , $ PublicBitFlags : ident) => { $ (# [$ outer]) * impl $ BitFlags { # [doc = " Yield a set of contained flags values."] # [doc = ""] # [doc = " Each yielded flags value will correspond to a defined named flag. Any unknown bits"] # [doc = " will be yielded together as a final flags value."] # [inline] pub const fn iter (& self) -> $ crate :: iter :: Iter <$ PublicBitFlags > { $ crate :: iter :: Iter :: __private_const_new (<$ PublicBitFlags as $ crate :: Flags >:: FLAGS , $ PublicBitFlags :: from_bits_retain (self . bits ()) , $ PublicBitFlags :: from_bits_retain (self . bits ()) ,) } # [doc = " Yield a set of contained named flags values."] # [doc = ""] # [doc = " This method is like [`iter`](#method.iter), except only yields bits in contained named flags."] # [doc = " Any unknown bits, or bits not corresponding to a contained flag will not be yielded."] # [inline] pub const fn iter_names (& self) -> $ crate :: iter :: IterNames <$ PublicBitFlags > { $ crate :: iter :: IterNames :: __private_const_new (<$ PublicBitFlags as $ crate :: Flags >:: FLAGS , $ PublicBitFlags :: from_bits_retain (self . bits ()) , $ PublicBitFlags :: from_bits_retain (self . bits ()) ,) } } $ (# [$ outer : meta]) * impl $ crate :: __private :: core :: iter :: IntoIterator for $ BitFlags { type Item = $ PublicBitFlags ; type IntoIter = $ crate :: iter :: Iter <$ PublicBitFlags >; fn into_iter (self) -> Self :: IntoIter { self . iter () } } } ; }
    };
}

__impl_public_bitflags_iter!()