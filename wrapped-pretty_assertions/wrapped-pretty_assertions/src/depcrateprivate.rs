// Generated macro for private (module)
macro_rules! Depcrateprivate {
() => {
// Module: crate
// Provides: {"private"}
// Dependencies: {}
# [doc (hidden)] pub mod private { # [cfg (feature = "alloc")] use alloc :: string :: String ; pub trait CompareAsStrByDefault : AsRef < str > { } impl CompareAsStrByDefault for str { } impl CompareAsStrByDefault for String { } impl < T : CompareAsStrByDefault + ? Sized > CompareAsStrByDefault for & T { } pub trait CreateComparison { type Comparison ; fn create_comparison (self) -> Self :: Comparison ; } impl < 'a , T , U > CreateComparison for & 'a (T , U) { type Comparison = crate :: Comparison < 'a , T , U > ; fn create_comparison (self) -> Self :: Comparison { crate :: Comparison :: new (& self . 0 , & self . 1) } } impl < 'a , T , U > CreateComparison for (& 'a T , & 'a U) where T : CompareAsStrByDefault + ? Sized , U : CompareAsStrByDefault + ? Sized , { type Comparison = crate :: StrComparison < 'a , T , U > ; fn create_comparison (self) -> Self :: Comparison { crate :: StrComparison :: new (self . 0 , self . 1) } } }
};
}
