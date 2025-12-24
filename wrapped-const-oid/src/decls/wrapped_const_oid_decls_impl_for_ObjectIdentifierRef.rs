use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ObjectIdentifierRef {
    /// Create an [`ObjectIdentifierRef`], validating that the provided byte slice contains a valid
    /// BER/DER encoding.
    pub fn from_bytes(ber: &[u8]) -> Result<&Self> {
        let mut arcs = Arcs::new(ber);
        while arcs.try_next()?.is_some() {}
        Ok(Self::from_bytes_unchecked(ber))
    }
    /// Create an [`ObjectIdentifierRef`] from the given byte slice without first checking that it
    /// contains valid BER/DER.
    pub(crate) const fn from_bytes_unchecked(ber: &[u8]) -> &Self {
        debug_assert!(! ber.is_empty());
        #[allow(unsafe_code)]
        unsafe { &*(ber as *const [u8] as *const ObjectIdentifierRef) }
    }
    /// Get the BER/DER serialization of this OID as bytes.
    ///
    /// Note that this encoding omits the ASN.1 tag/length, and only contains the value portion of
    /// the encoded OID.
    pub const fn as_bytes(&self) -> &[u8] {
        &self.ber
    }
    /// Return the arc with the given index, if it exists.
    pub fn arc(&self, index: usize) -> Option<Arc> {
        self.arcs().nth(index)
    }
    /// Iterate over the arcs (a.k.a. nodes) of an [`ObjectIdentifier`].
    ///
    /// Returns [`Arcs`], an iterator over [`Arc`] values.
    pub fn arcs(&self) -> Arcs<'_> {
        Arcs::new(self.ber.as_ref())
    }
    /// Get the length of this [`ObjectIdentifier`] in arcs.
    pub fn len(&self) -> usize {
        self.arcs().count()
    }
}
