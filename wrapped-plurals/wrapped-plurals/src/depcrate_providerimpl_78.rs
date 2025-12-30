// Generated macro for impl_78 (impl)
macro_rules! Depcrate_providerimpl_78 {
() => {
// Module: crate::provider
// Provides: {"impl_78"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl < 'a , V > PluralElementsInner < (FourBitMetadata , & 'a V) > where V : VarULE + ? Sized , { fn from_packed (packed : & 'a PluralElementsPackedULE < V >) -> Self { let parts = packed . as_parts () ; PluralElementsInner { other : parts . default , zero : parts . specials . and_then (| specials | get_special (specials , PluralElementsKeys :: Zero)) , one : parts . specials . and_then (| specials | get_special (specials , PluralElementsKeys :: One)) , two : parts . specials . and_then (| specials | get_special (specials , PluralElementsKeys :: Two)) , few : parts . specials . and_then (| specials | get_special (specials , PluralElementsKeys :: Few)) , many : parts . specials . and_then (| specials | get_special (specials , PluralElementsKeys :: Many)) , explicit_zero : parts . specials . and_then (| specials | get_special (specials , PluralElementsKeys :: ExplicitZero)) , explicit_one : parts . specials . and_then (| specials | get_special (specials , PluralElementsKeys :: ExplicitOne)) , } } }
};
}
