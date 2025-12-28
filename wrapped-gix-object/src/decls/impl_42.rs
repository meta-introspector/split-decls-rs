macro_rules! deps {
    () => {
        Commit!();
        Kind!();
        Write!();
        WriteTo!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl crate :: WriteTo for Commit { # [doc = " Serializes this instance to `out` in the git serialization format."] fn write_to (& self , mut out : & mut dyn io :: Write) -> io :: Result < () > { encode :: trusted_header_id (b"tree" , & self . tree , & mut out) ? ; for parent in & self . parents { encode :: trusted_header_id (b"parent" , parent , & mut out) ? ; } let mut buf = gix_date :: parse :: TimeBuf :: default () ; encode :: trusted_header_signature (b"author" , & self . author . to_ref (& mut buf) , & mut out) ? ; encode :: trusted_header_signature (b"committer" , & self . committer . to_ref (& mut buf) , & mut out) ? ; if let Some (encoding) = self . encoding . as_ref () { encode :: header_field (b"encoding" , encoding , & mut out) ? ; } for (name , value) in & self . extra_headers { encode :: header_field_multi_line (name , value , & mut out) ? ; } out . write_all (NL) ? ; out . write_all (& self . message) } fn kind (& self) -> Kind { Kind :: Commit } fn size (& self) -> u64 { let hash_in_hex = self . tree . kind () . len_in_hex () ; (b"tree" . len () + 1 + hash_in_hex + 1 + self . parents . iter () . count () * (b"parent" . len () + 1 + hash_in_hex + 1) + b"author" . len () + 1 + self . author . size () + 1 + b"committer" . len () + 1 + self . committer . size () + 1 + self . encoding . as_ref () . map_or (0 , | e | b"encoding" . len () + 1 + e . len () + 1) + self . extra_headers . iter () . map (| (name , value) | { name . len () + value . lines_with_terminator () . map (| s | s . len () + 1) . sum :: < usize > () + usize :: from (! value . ends_with_str (b"\n")) }) . sum :: < usize > () + 1 + self . message . len ()) as u64 } }
    };
}

impl_42!()