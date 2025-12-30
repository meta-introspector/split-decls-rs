// Generated macro for HasSubset (trait)
macro_rules! DepcrateHasSubset {
() => {
// Module: crate
// Provides: {"HasSubset"}
// Dependencies: {}
# [doc = " *(internal)* Asserts that the parts of the partial reference `Reference` are a subset of the"] # [doc = " parts of the partial reference having this trait."] # [doc = ""] # [doc = " A list of parts is considered a subset if they can be plucked in sequence."] pub unsafe trait HasSubset < 'a , Reference , SubsetIndex > : PartialRef < 'a > { type Remainder : PartialRef < 'a , Target = Self :: Target > ; }
};
}
