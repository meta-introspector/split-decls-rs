// Generated macro for invert_batch_internal (function)
macro_rules! Depcrate_opsinvert_batch_internal {
() => {
// Module: crate::ops
// Provides: {"invert_batch_internal"}
// Dependencies: {}
# [doc = " Implements \"Montgomery's trick\", a trick for computing many modular inverses at once."] # [doc = ""] # [doc = " \"Montgomery's trick\" works by reducing the problem of computing `n` inverses"] # [doc = " to computing a single inversion, plus some storage and `O(n)` extra multiplications."] # [doc = ""] # [doc = " See: https://iacr.org/archive/pkc2004/29470042/29470042.pdf section 2.2."] pub (crate) fn invert_batch_internal < T : Copy + Mul < Output = T > + MulAssign > (field_elements : & mut [T] , field_elements_pad : & mut [T] , invert : fn (T) -> (T , Choice) ,) -> Choice { let batch_size = field_elements . len () ; if batch_size != field_elements_pad . len () { return Choice :: from (0) ; } if batch_size == 0 { return Choice :: from (1) ; } let mut acc = field_elements [0] ; field_elements_pad [0] = acc ; for (field_element , field_element_pad) in field_elements . iter_mut () . zip (field_elements_pad . iter_mut ()) . skip (1) { acc *= * field_element ; * field_element_pad = acc ; } let (mut acc , choice) = invert (acc) ; let field_elements_pad = field_elements_pad . iter () . rev () . skip (1) . map (Some) . chain (iter :: once (None)) ; for (field_element , field_element_pad) in field_elements . iter_mut () . rev () . zip (field_elements_pad) { if let Some (field_element_pad) = field_element_pad { let tmp = acc * * field_element ; * field_element = acc * * field_element_pad ; acc = tmp ; } else { * field_element = acc ; } } choice }
};
}
