// MINIMAL TEST CASE for parsing failure in: ../rust/library/portable-simd/crates/core_simd/examples/dot_product.rs
// Error: expected square brackets
// Problematic line: line 15

// go along the resulting array and add up the result.
// In the next example we will see if there
//  is any difference to adding and multiplying in tandem.
pub fn dot_prod_scalar_0(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());

    a.iter().zip(b.iter()).map(|(a, b)| a * b).sum()
