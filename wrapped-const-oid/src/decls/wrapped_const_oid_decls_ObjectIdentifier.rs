use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Object identifier (OID).
///
/// OIDs are hierarchical structures consisting of "arcs", i.e. integer
/// identifiers.
///
/// # Validity
///
/// In order for an OID to be considered valid by this library, it must meet
/// the following criteria:
///
/// - The OID MUST have at least 3 arcs
/// - The first arc MUST be within the range 0-2
/// - The second arc MUST be within the range 0-39
/// - The BER/DER encoding of the OID MUST be shorter than
///   [`ObjectIdentifier::MAX_SIZE`]
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ObjectIdentifier<const MAX_SIZE: usize = DEFAULT_MAX_SIZE> {
    /// Buffer containing BER/DER-serialized bytes (sans ASN.1 tag/length)
    ber: Buffer<MAX_SIZE>,
}
