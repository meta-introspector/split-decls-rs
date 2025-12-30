// Generated macro for impl_quickcheck (module)
macro_rules! Depcrate_arbitraryimpl_quickcheck {
() => {
// Module: crate::arbitrary
// Provides: {"impl_quickcheck"}
// Dependencies: {}
# [cfg (feature = "quickcheck")] # [cfg_attr (docsrs , doc (cfg (feature = "quickcheck")))] mod impl_quickcheck { use crate :: { IndexMap , IndexSet } ; use alloc :: boxed :: Box ; use alloc :: vec :: Vec ; use core :: hash :: { BuildHasher , Hash } ; use quickcheck :: { Arbitrary , Gen } ; impl < K , V , S > Arbitrary for IndexMap < K , V , S > where K : Arbitrary + Hash + Eq , V : Arbitrary , S : BuildHasher + Default + Clone + 'static , { fn arbitrary (g : & mut Gen) -> Self { Self :: from_iter (Vec :: arbitrary (g)) } fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { let vec = Vec :: from_iter (self . clone ()) ; Box :: new (vec . shrink () . map (Self :: from_iter)) } } impl < T , S > Arbitrary for IndexSet < T , S > where T : Arbitrary + Hash + Eq , S : BuildHasher + Default + Clone + 'static , { fn arbitrary (g : & mut Gen) -> Self { Self :: from_iter (Vec :: arbitrary (g)) } fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { let vec = Vec :: from_iter (self . clone ()) ; Box :: new (vec . shrink () . map (Self :: from_iter)) } } }
};
}
