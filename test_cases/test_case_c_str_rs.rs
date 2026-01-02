// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/ffi/c_str.rs
// Error: expected square brackets
// Problematic line: line 18

//   actually reference libstd or liballoc in intra-doc links. so, the best we can do is remove the
//   links to `CString` and `String` for now until a solution is developed

/// Representation of a borrowed C string.
///
/// This type represents a borrowed reference to a nul-terminated
/// array of bytes. It can be constructed safely from a <code>&[[u8]]</code>
