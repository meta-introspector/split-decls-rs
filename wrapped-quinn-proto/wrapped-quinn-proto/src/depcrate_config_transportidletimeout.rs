// Generated macro for IdleTimeout (struct)
macro_rules! Depcrate_config_transportIdleTimeout {
() => {
// Module: crate::config::transport
// Provides: {"IdleTimeout"}
// Dependencies: {}
# [doc = " Maximum duration of inactivity to accept before timing out the connection"] # [doc = ""] # [doc = " This wraps an underlying [`VarInt`], representing the duration in milliseconds. Values can be"] # [doc = " constructed by converting directly from `VarInt`, or using `TryFrom<Duration>`."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::{convert::TryFrom, time::Duration};"] # [doc = " # use quinn_proto::{IdleTimeout, VarIntBoundsExceeded, VarInt};"] # [doc = " # fn main() -> Result<(), VarIntBoundsExceeded> {"] # [doc = " // A `VarInt`-encoded value in milliseconds"] # [doc = " let timeout = IdleTimeout::from(VarInt::from_u32(10_000));"] # [doc = ""] # [doc = " // Try to convert a `Duration` into a `VarInt`-encoded timeout"] # [doc = " let timeout = IdleTimeout::try_from(Duration::from_secs(10))?;"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] # [derive (Default , Copy , Clone , Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct IdleTimeout (VarInt) ;
};
}
