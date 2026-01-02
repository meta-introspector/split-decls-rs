// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/uefi/helpers.rs
// Error: expected square brackets
// Problematic line: line 28

use crate::sync::atomic::{Atomic, AtomicPtr, Ordering};
use crate::sys_common::wstr::WStrUnits;

type BootInstallMultipleProtocolInterfaces =
    unsafe extern "efiapi" fn(_: *mut r_efi::efi::Handle, _: ...) -> r_efi::efi::Status;

type BootUninstallMultipleProtocolInterfaces =
