macro_rules! IntraPackLookup {
    () => {
        # [doc = " A utility to allow looking up pack offsets for a particular pack"] pub (crate) enum IntraPackLookup < 'a > { Single (& 'a gix_pack :: index :: File) , # [doc = " the internal pack-id inside of a multi-index for which the lookup is supposed to be."] # [doc = " Used to prevent ref-delta OIDs to, for some reason, point to a different pack."] Multi { index : & 'a gix_pack :: multi_index :: File , required_pack_index : gix_pack :: multi_index :: PackIndex , } , }
    };
}

IntraPackLookup!();