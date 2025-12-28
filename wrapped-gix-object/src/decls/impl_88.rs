macro_rules! deps {
    () => {
        Tag!();
        WriteTo!();
        TagRef!();
        Write!();
        Kind!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl crate :: WriteTo for TagRef < '_ > { fn write_to (& self , mut out : & mut dyn io :: Write) -> io :: Result < () > { encode :: trusted_header_field (b"object" , self . target , & mut out) ? ; encode :: trusted_header_field (b"type" , self . target_kind . as_bytes () , & mut out) ? ; encode :: header_field (b"tag" , validated_name (self . name) ? , & mut out) ? ; if let Some (tagger) = self . tagger { encode :: trusted_header_field (b"tagger" , tagger . as_ref () , & mut out) ? ; } if ! self . message . iter () . all (| b | * b == b'\n') { out . write_all (NL) ? ; } out . write_all (self . message) ? ; if let Some (message) = self . pgp_signature { out . write_all (NL) ? ; out . write_all (message) ? ; } Ok (()) } fn kind (& self) -> Kind { Kind :: Tag } fn size (& self) -> u64 { (b"object" . len () + 1 + self . target () . kind () . len_in_hex () + 1 + b"type" . len () + 1 + self . target_kind . as_bytes () . len () + 1 + b"tag" . len () + 1 + self . name . len () + 1 + self . tagger . map_or (0 , | raw | b"tagger" . len () + 1 + raw . len () + 1) + if self . message . iter () . all (| b | * b == b'\n') { 0 } else { 1 } + self . message . len () + self . pgp_signature . as_ref () . map_or (0 , | m | 1 + m . len ())) as u64 } }
    };
}

impl_88!()