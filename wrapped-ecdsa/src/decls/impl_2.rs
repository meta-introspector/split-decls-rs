macro_rules! deps {
    () => {
        RecoveryId!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl RecoveryId { # [doc = " Maximum supported value for the recovery ID (inclusive)."] pub const MAX : u8 = 3 ; # [doc = " Create a new [`RecoveryId`] from the following 1-bit arguments:"] # [doc = ""] # [doc = " - `is_y_odd`: is the affine y-coordinate of 𝑘×𝑮 odd?"] # [doc = " - `is_x_reduced`: did the affine x-coordinate of 𝑘×𝑮 overflow the curve order?"] pub const fn new (is_y_odd : bool , is_x_reduced : bool) -> Self { Self (((is_x_reduced as u8) << 1) | (is_y_odd as u8)) } # [doc = " Did the affine x-coordinate of 𝑘×𝑮 overflow the curve order?"] pub const fn is_x_reduced (self) -> bool { (self . 0 & 0b10) != 0 } # [doc = " Is the affine y-coordinate of 𝑘×𝑮 odd?"] pub const fn is_y_odd (self) -> bool { (self . 0 & 1) != 0 } # [doc = " Convert a `u8` into a [`RecoveryId`]."] pub const fn from_byte (byte : u8) -> Option < Self > { if byte <= Self :: MAX { Some (Self (byte)) } else { None } } # [doc = " Convert this [`RecoveryId`] into a `u8`."] pub const fn to_byte (self) -> u8 { self . 0 } }
    };
}

impl_2!();