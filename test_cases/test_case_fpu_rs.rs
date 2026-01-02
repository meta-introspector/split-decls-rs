// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/dec2flt/fpu.rs
// Error: expected square brackets
// Problematic line: line 22

// (If this is used to set 32bit precision, there is still a risk that the compiler moves some 64bit
// operation into the scope of the `set_precision` guard. So it's not like this is totally sound.
// But it's not really any less sound than the default state of 80bit precision...)
#[cfg(all(target_arch = "x86", not(target_feature = "sse2")))]
mod fpu_precision {
    use core::arch::asm;

