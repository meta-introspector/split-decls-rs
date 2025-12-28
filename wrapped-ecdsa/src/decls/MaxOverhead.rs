macro_rules! MaxOverhead {
    () => {
        # [doc = " Maximum overhead of an ASN.1 DER-encoded ECDSA signature for a given curve:"] # [doc = " 9-bytes."] # [doc = ""] # [doc = " Includes 3-byte ASN.1 DER header:"] # [doc = ""] # [doc = " - 1-byte: ASN.1 `SEQUENCE` tag (0x30)"] # [doc = " - 2-byte: length"] # [doc = ""] # [doc = " ...followed by two ASN.1 `INTEGER` values, which each have a header whose"] # [doc = " maximum length is the following:"] # [doc = ""] # [doc = " - 1-byte: ASN.1 `INTEGER` tag (0x02)"] # [doc = " - 1-byte: length"] # [doc = " - 1-byte: zero to indicate value is positive (`INTEGER` is signed)"] pub type MaxOverhead = U9 ;
    };
}

MaxOverhead!()