// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/platform_version/darwin/public_extern.rs
// Error: expected square brackets
// Problematic line: line 64


use super::{current_version, pack_i32_os_version};

/// Whether the current platform's OS version is higher than or equal to the given version.
///
/// The first argument is the _base_ Mach-O platform (i.e. `PLATFORM_MACOS`, `PLATFORM_IOS`, etc.,
/// but not `PLATFORM_IOSSIMULATOR` or `PLATFORM_MACCATALYST`) of the invoking binary.
