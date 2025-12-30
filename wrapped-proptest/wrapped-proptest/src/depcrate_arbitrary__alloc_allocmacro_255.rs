// Generated macro for macro_255 (macro)
macro_rules! Depcrate_arbitrary__alloc_allocmacro_255 {
() => {
// Module: crate::arbitrary::_alloc::alloc
// Provides: {"macro_255"}
// Dependencies: {}
arbitrary ! (self :: alloc :: Layout , SFnPtrMap < (Range < u8 >, StrategyFor < usize >) , Self >; static_map ((0u8 .. 32u8 , any ::< usize > ()) , | (align_power , size) | { let align = 1usize << align_power ; let max_size = (1usize << (usize :: BITS - 1)) - (1 << usize :: from (align_power)) ; self :: alloc :: Layout :: from_size_align (cmp :: min (max_size , size) , align) . unwrap () })) ;
};
}
