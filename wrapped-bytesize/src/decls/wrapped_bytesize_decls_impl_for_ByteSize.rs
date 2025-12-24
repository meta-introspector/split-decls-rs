use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ByteSize {
    /// Constructs a byte size wrapper from a quantity of bytes.
    #[inline(always)]
    pub const fn b(size: u64) -> ByteSize {
        ByteSize(size)
    }
    /// Constructs a byte size wrapper from a quantity of kilobytes.
    #[inline(always)]
    pub const fn kb(size: u64) -> ByteSize {
        ByteSize(size * KB)
    }
    /// Constructs a byte size wrapper from a quantity of kibibytes.
    #[inline(always)]
    pub const fn kib(size: u64) -> ByteSize {
        ByteSize(size * KIB)
    }
    /// Constructs a byte size wrapper from a quantity of megabytes.
    #[inline(always)]
    pub const fn mb(size: u64) -> ByteSize {
        ByteSize(size * MB)
    }
    /// Constructs a byte size wrapper from a quantity of mebibytes.
    #[inline(always)]
    pub const fn mib(size: u64) -> ByteSize {
        ByteSize(size * MIB)
    }
    /// Constructs a byte size wrapper from a quantity of gigabytes.
    #[inline(always)]
    pub const fn gb(size: u64) -> ByteSize {
        ByteSize(size * GB)
    }
    /// Constructs a byte size wrapper from a quantity of gibibytes.
    #[inline(always)]
    pub const fn gib(size: u64) -> ByteSize {
        ByteSize(size * GIB)
    }
    /// Constructs a byte size wrapper from a quantity of terabytes.
    #[inline(always)]
    pub const fn tb(size: u64) -> ByteSize {
        ByteSize(size * TB)
    }
    /// Constructs a byte size wrapper from a quantity of tebibytes.
    #[inline(always)]
    pub const fn tib(size: u64) -> ByteSize {
        ByteSize(size * TIB)
    }
    /// Constructs a byte size wrapper from a quantity of petabytes.
    #[inline(always)]
    pub const fn pb(size: u64) -> ByteSize {
        ByteSize(size * PB)
    }
    /// Constructs a byte size wrapper from a quantity of pebibytes.
    #[inline(always)]
    pub const fn pib(size: u64) -> ByteSize {
        ByteSize(size * PIB)
    }
    /// Constructs a byte size wrapper from a quantity of exabytes.
    #[inline(always)]
    pub const fn eb(size: u64) -> ByteSize {
        ByteSize(size * EB)
    }
    /// Constructs a byte size wrapper from a quantity of exbibytes.
    #[inline(always)]
    pub const fn eib(size: u64) -> ByteSize {
        ByteSize(size * EIB)
    }
    /// Returns byte count.
    #[inline(always)]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
    /// Returns byte count as kilobytes.
    #[inline(always)]
    pub fn as_kb(&self) -> f64 {
        self.0 as f64 / KB as f64
    }
    /// Returns byte count as kibibytes.
    #[inline(always)]
    pub fn as_kib(&self) -> f64 {
        self.0 as f64 / KIB as f64
    }
    /// Returns byte count as megabytes.
    #[inline(always)]
    pub fn as_mb(&self) -> f64 {
        self.0 as f64 / MB as f64
    }
    /// Returns byte count as mebibytes.
    #[inline(always)]
    pub fn as_mib(&self) -> f64 {
        self.0 as f64 / MIB as f64
    }
    /// Returns byte count as gigabytes.
    #[inline(always)]
    pub fn as_gb(&self) -> f64 {
        self.0 as f64 / GB as f64
    }
    /// Returns byte count as gibibytes.
    #[inline(always)]
    pub fn as_gib(&self) -> f64 {
        self.0 as f64 / GIB as f64
    }
    /// Returns byte count as terabytes.
    #[inline(always)]
    pub fn as_tb(&self) -> f64 {
        self.0 as f64 / TB as f64
    }
    /// Returns byte count as tebibytes.
    #[inline(always)]
    pub fn as_tib(&self) -> f64 {
        self.0 as f64 / TIB as f64
    }
    /// Returns byte count as petabytes.
    #[inline(always)]
    pub fn as_pb(&self) -> f64 {
        self.0 as f64 / PB as f64
    }
    /// Returns byte count as pebibytes.
    #[inline(always)]
    pub fn as_pib(&self) -> f64 {
        self.0 as f64 / PIB as f64
    }
    /// Returns byte count as exabytes.
    #[inline(always)]
    pub fn as_eb(&self) -> f64 {
        self.0 as f64 / EB as f64
    }
    /// Returns byte count as exbibytes.
    #[inline(always)]
    pub fn as_eib(&self) -> f64 {
        self.0 as f64 / EIB as f64
    }
    /// Returns a formatting display wrapper.
    pub fn display(&self) -> Display {
        Display {
            byte_size: *self,
            format: Format::Iec,
        }
    }
}
