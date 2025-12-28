macro_rules! deps {
    () => {
        StableHasher!();
    };
}

macro_rules! ExtendedHasher {
    () => {
        deps!();
        # [doc = " Extended [`Hasher`] trait for use with [`StableHasher`]."] # [doc = ""] # [doc = " It permits returning an arbitrary type as the [`Self::Hash`] type"] # [doc = " contrary to the [`Hasher`] trait which can only return `u64`. This"] # [doc = " is useful when the hasher uses a different representation."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::hash::Hasher;"] # [doc = " use rustc_stable_hash::ExtendedHasher;"] # [doc = ""] # [doc = " struct BogusHasher(u128);"] # [doc = ""] # [doc = " impl Hasher for BogusHasher {"] # [doc = "     fn write(&mut self, a: &[u8]) {"] # [doc = "         # self.0 = a.iter().fold(0u128, |acc, a| acc + (*a as u128)) + self.0;"] # [doc = "         // ..."] # [doc = "     }"] # [doc = ""] # [doc = "     fn finish(&self) -> u64 {"] # [doc = "         self.0 as u64 // really bogus"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " impl ExtendedHasher for BogusHasher {"] # [doc = "     type Hash = u128;"] # [doc = ""] # [doc = "     fn short_write<const LEN: usize>(&mut self, bytes: [u8; LEN]) {"] # [doc = "         self.write(&bytes)"] # [doc = "     }"] # [doc = ""] # [doc = "     fn finish(self) -> Self::Hash {"] # [doc = "         self.0"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub trait ExtendedHasher : Hasher { # [doc = " Type returned by the hasher."] type Hash ; # [doc = " Optimized version of [`Hasher::write`] but for small write."] fn short_write < const LEN : usize > (& mut self , bytes : [u8 ; LEN]) { self . write (& bytes) ; } # [doc = " Finalization method of the hasher to return the [`Hash`]."] fn finish (self) -> Self :: Hash ; }
    };
}

ExtendedHasher!();