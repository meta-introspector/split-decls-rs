// MINIMAL TEST CASE for parsing failure in: ../rust/library/backtrace/src/symbolize/gimli/elf.rs
// Error: expected square brackets
// Problematic line: line 14

use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::str;
#[cfg(feature = "ruzstd")]
use object::elf::ELFCOMPRESS_ZSTD;
use object::elf::{ELF_NOTE_GNU, ELFCOMPRESS_ZLIB, NT_GNU_BUILD_ID, SHF_COMPRESSED};
use object::read::StringTable;
