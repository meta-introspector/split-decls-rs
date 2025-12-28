macro_rules! deps {
    () => {
        Error!();
        Version!();
        Repository!();
        Pack!();
    };
}

macro_rules! pack_index_version {
    () => {
        deps!();
        pub fn pack_index_version (repo : & Repository) -> Result < gix_pack :: index :: Version , Error > { Ok (repo . config . resolved . integer (Pack :: INDEX_VERSION) . map (| value | Pack :: INDEX_VERSION . try_into_index_version (value)) . transpose () . with_leniency (repo . options . lenient_config) ? . unwrap_or (gix_pack :: index :: Version :: V2)) }
    };
}

pack_index_version!()