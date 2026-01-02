// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_data_structures/src/flock.rs
// Error: expected square brackets
// Problematic line: line 7

//! green/native threading. This is just a bare-bones enough solution for
//! librustdoc, it is not production quality at all.

cfg_select! {
    target_os = "linux" => {
        mod linux;
        use linux as imp;
