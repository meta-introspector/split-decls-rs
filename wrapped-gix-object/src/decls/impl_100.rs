macro_rules! deps {
    () => {
        Error!();
        Tag!();
        TagRef!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < 'a > TagRef < 'a > { # [doc = " Deserialize a tag from `data`."] pub fn from_bytes (mut data : & 'a [u8]) -> Result < TagRef < 'a > , crate :: decode :: Error > { let input = & mut data ; match decode :: git_tag . parse_next (input) { Ok (tag) => Ok (tag) , Err (err) => Err (crate :: decode :: Error :: with_err (err , input)) , } } # [doc = " The object this tag points to as `Id`."] pub fn target (& self) -> gix_hash :: ObjectId { gix_hash :: ObjectId :: from_hex (self . target) . expect ("prior validation") } # [doc = " Return the tagger, if present."] pub fn tagger (& self) -> Result < Option < gix_actor :: SignatureRef < 'a > > , crate :: decode :: Error > { Ok (self . tagger . map (parse_signature) . transpose () ? . map (| signature | signature . trim ())) } # [doc = " Copy all data into a fully-owned instance."] pub fn into_owned (self) -> Result < crate :: Tag , crate :: decode :: Error > { self . try_into () } }
    };
}

impl_100!();