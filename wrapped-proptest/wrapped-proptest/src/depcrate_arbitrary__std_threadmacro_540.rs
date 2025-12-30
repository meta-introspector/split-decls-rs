// Generated macro for macro_540 (macro)
macro_rules! Depcrate_arbitrary__std_threadmacro_540 {
() => {
// Module: crate::arbitrary::_std::thread
// Provides: {"macro_540"}
// Dependencies: {}
arbitrary ! (Builder , SMapped < (Option < usize >, Option < String >) , Self >; { let prob = prob (0.7) ; let args = product_pack ! [product_pack ! [prob , Default :: default ()] , product_pack ! [prob , Default :: default ()]] ; static_map (arbitrary_with (args) , | (os , on) | { let mut b = Builder :: new () ; b = if let Some (size) = os { b . stack_size (size) } else { b } ; if let Some (name) = on { b . name (name) } else { b } }) }) ;
};
}
