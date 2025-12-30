// Generated macro for CaS (trait)
macro_rules! Depcrate_strategyCaS {
() => {
// Module: crate::strategy
// Provides: {"CaS"}
// Dependencies: {}
# [doc = " An extension of the [`Strategy`], allowing for compare and swap operation."] # [doc = ""] # [doc = " The compare and swap operation is \"advanced\" and not all strategies need to support them."] # [doc = " Therefore, it is a separate trait."] # [doc = ""] # [doc = " Similarly, it is not yet made publicly usable or implementable and works only as a bound."] pub trait CaS < T : RefCnt > : sealed :: CaS < T > { }
};
}
