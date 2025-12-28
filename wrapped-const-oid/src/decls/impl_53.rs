macro_rules! deps {
    () => {
        ObjectIdentifierRef!();
        Arc!();
        Result!();
        Encoder!();
        ObjectIdentifier!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < const MAX_SIZE : usize > ObjectIdentifier < MAX_SIZE > { # [doc = " Parse an OID from from its BER/DER encoding."] # [doc = ""] # [doc = " Returns `Err(Error::Length)` if bytes do not fit in `MAX_SIZE`."] pub fn from_bytes_sized (ber_bytes : & [u8]) -> Result < Self > { ObjectIdentifierRef :: from_bytes (ber_bytes) ? . try_into () } # [doc = " Get the BER/DER serialization of this OID as bytes."] # [doc = ""] # [doc = " Note that this encoding omits the ASN.1 tag/length, and only contains the value portion of"] # [doc = " the encoded OID."] pub const fn as_bytes (& self) -> & [u8] { self . ber . as_bytes () } # [doc = " Borrow an [`ObjectIdentifierRef`] which corresponds to this [`ObjectIdentifier`]."] pub const fn as_oid_ref (& self) -> & ObjectIdentifierRef { ObjectIdentifierRef :: from_bytes_unchecked (self . as_bytes ()) } # [doc = " Get the parent OID of this one (if applicable)."] pub fn parent (& self) -> Option < Self > { let num_arcs = self . len () . checked_sub (1) ? ; let mut encoder = Encoder :: new () ; for arc in self . arcs () . take (num_arcs) { encoder = encoder . arc (arc) . ok () ? ; } encoder . finish () . ok () } # [doc = " Push an additional arc onto this OID, returning the child OID."] pub const fn push_arc (self , arc : Arc) -> Result < Self > { match Encoder :: extend (self) . arc (arc) { Ok (encoder) => encoder . finish () , Err (err) => Err (err) , } } # [doc = " Does this OID start with the other OID?"] pub const fn starts_with < const SIZE : usize > (& self , other : ObjectIdentifier < SIZE >) -> bool { let len = other . as_bytes () . len () ; if self . as_bytes () . len () < len { return false ; } let mut i = 0 ; while i < len { if self . as_bytes () [i] != other . as_bytes () [i] { return false ; } match i . checked_add (1) { Some (succ) => i = succ , None => return false , } } true } }
    };
}

impl_53!();