// Generated macro for buffer (macro)
macro_rules! Depcrate_arbitrary__std_iobuffer {
() => {
// Module: crate::arbitrary::_std::io
// Provides: {"buffer"}
// Dependencies: {}
macro_rules ! buffer { ($ type : ident , $ bound : path) => { arbitrary ! ([A : Arbitrary + $ bound] $ type < A >, SMapped < (A , Option < u16 >) , Self >, A :: Parameters ; args => static_map (arbitrary_with (product_pack ! [args , Default :: default ()]) , | (inner , cap) | { if let Some (cap) = cap { $ type :: with_capacity (cap as usize , inner) } else { $ type :: new (inner) } })) ; lift1 ! ([$ bound] $ type < A >; base => (base , any ::< Option < u16 >> ()) . prop_map (| (inner , cap) | { if let Some (cap) = cap { $ type :: with_capacity (cap as usize , inner) } else { $ type :: new (inner) } })) ; } ; }
};
}
