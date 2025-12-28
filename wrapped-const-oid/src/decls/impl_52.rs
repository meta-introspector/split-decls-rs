macro_rules! deps {
    () => {
        Result!();
        ObjectIdentifier!();
        Arc!();
        Parser!();
        Encoder!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl ObjectIdentifier { # [doc = " Maximum size of a BER/DER-encoded OID in bytes."] pub const MAX_SIZE : usize = DEFAULT_MAX_SIZE ; # [doc = " Parse an [`ObjectIdentifier`] from the dot-delimited string form,"] # [doc = " panicking on parse errors."] # [doc = ""] # [doc = " This function exists as a workaround for `unwrap` not yet being"] # [doc = " stable in `const fn` contexts, and is intended to allow the result to"] # [doc = " be bound to a constant value:"] # [doc = ""] # [doc = " ```"] # [doc = " use const_oid::ObjectIdentifier;"] # [doc = ""] # [doc = " pub const MY_OID: ObjectIdentifier = ObjectIdentifier::new_unwrap(\"1.2.840.113549.1.1.1\");"] # [doc = " ```"] # [doc = ""] # [doc = " In future versions of Rust it should be possible to replace this with"] # [doc = " `ObjectIdentifier::new(...).unwrap()`."] # [doc = ""] # [doc = " Use [`ObjectIdentifier::new`] for fallible parsing."] pub const fn new_unwrap (s : & str) -> Self { match Self :: new (s) { Ok (oid) => oid , Err (err) => err . panic () , } } # [doc = " Parse an [`ObjectIdentifier`] from the dot-delimited string form."] pub const fn new (s : & str) -> Result < Self > { match parser :: Parser :: parse (s) { Ok (parser) => parser . finish () , Err (err) => Err (err) , } } # [doc = " Parse an OID from a slice of [`Arc`] values (i.e. integers)."] pub fn from_arcs (arcs : impl IntoIterator < Item = Arc >) -> Result < Self > { let mut encoder = Encoder :: new () ; for arc in arcs { encoder = encoder . arc (arc) ? ; } encoder . finish () } # [doc = " Parse an OID from from its BER/DER encoding."] pub fn from_bytes (ber_bytes : & [u8]) -> Result < Self > { Self :: from_bytes_sized (ber_bytes) } }
    };
}

impl_52!();