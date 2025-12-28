macro_rules! deps {
    () => {
        Arc!();
        ObjectIdentifier!();
    };
}

macro_rules! Arcs {
    () => {
        deps!();
        # [doc = " [`Iterator`] over [`Arc`] values (a.k.a. nodes) in an [`ObjectIdentifier`]."] # [doc = ""] # [doc = " This iterates over all arcs in an OID, including the root."] pub struct Arcs < 'a > { # [doc = " OID bytes we're iterating over."] bytes : & 'a [u8] , # [doc = " Current position within the serialized BER bytes of this OID."] cursor : Option < usize > , }
    };
}

Arcs!()