use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Marker trait for elliptic curves intended for use with ECDSA.
pub trait EcdsaCurve: PrimeCurve {
    /// Does this curve use low-S normalized signatures?
    ///
    /// This is typically `false`. See [`Signature::normalize_s`] for more information.
    const NORMALIZE_S: bool;
}
