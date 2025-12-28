macro_rules! deps {
    () => {
        WriteTo!();
        Tag!();
        Kind!();
        Write!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl crate :: WriteTo for Tag { fn write_to (& self , out : & mut dyn io :: Write) -> io :: Result < () > { encode :: trusted_header_id (b"object" , & self . target , out) ? ; encode :: trusted_header_field (b"type" , self . target_kind . as_bytes () , out) ? ; encode :: header_field (b"tag" , validated_name (self . name . as_ref ()) ? , out) ? ; if let Some (tagger) = & self . tagger { let mut buf = TimeBuf :: default () ; encode :: trusted_header_signature (b"tagger" , & tagger . to_ref (& mut buf) , out) ? ; } if ! self . message . iter () . all (| b | * b == b'\n') { out . write_all (NL) ? ; } out . write_all (self . message . as_ref ()) ? ; if let Some (message) = & self . pgp_signature { out . write_all (NL) ? ; out . write_all (message . as_ref ()) ? ; } Ok (()) } fn kind (& self) -> Kind { Kind :: Tag } fn size (& self) -> u64 { (b"object" . len () + 1 + self . target . kind () . len_in_hex () + 1 + b"type" . len () + 1 + self . target_kind . as_bytes () . len () + 1 + b"tag" . len () + 1 + self . name . len () + 1 + self . tagger . as_ref () . map_or (0 , | t | b"tagger" . len () + 1 + t . size () + 1) + if self . message . iter () . all (| b | * b == b'\n') { 0 } else { 1 } + self . message . len () + self . pgp_signature . as_ref () . map_or (0 , | m | 1 + m . len ())) as u64 } }
    };
}

impl_87!()