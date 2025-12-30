// Generated macro for impl_59 (impl)
macro_rules! Depcrate_providerimpl_59 {
() => {
// Module: crate::provider
// Provides: {"impl_59"}
// Dependencies: {}
impl < T > PluralElementsInner < T > where T : PartialEq , { fn get_specials_tuples (& self) -> impl Iterator < Item = (PluralElementsKeys , & T) > { [self . zero . as_ref () . filter (| & p | * p != self . other) . map (| s | (PluralElementsKeys :: Zero , s)) , self . one . as_ref () . filter (| & p | * p != self . other) . map (| s | (PluralElementsKeys :: One , s)) , self . two . as_ref () . filter (| & p | * p != self . other) . map (| s | (PluralElementsKeys :: Two , s)) , self . few . as_ref () . filter (| & p | * p != self . other) . map (| s | (PluralElementsKeys :: Few , s)) , self . many . as_ref () . filter (| & p | * p != self . other) . map (| s | (PluralElementsKeys :: Many , s)) , self . explicit_zero . as_ref () . filter (| & p | * p != self . other) . map (| s | (PluralElementsKeys :: ExplicitZero , s)) , self . explicit_one . as_ref () . filter (| & p | * p != self . other) . map (| s | (PluralElementsKeys :: ExplicitOne , s)) ,] . into_iter () . flatten () } }
};
}
