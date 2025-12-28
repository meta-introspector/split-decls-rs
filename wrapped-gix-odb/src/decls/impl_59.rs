macro_rules! deps {
    () => {
        IntraPackLookup!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl IntraPackLookup < '_ > { pub (crate) fn pack_offset_by_id (& self , id : & oid) -> Option < gix_pack :: data :: Offset > { match self { IntraPackLookup :: Single (index) => index . lookup (id) . map (| entry_index | index . pack_offset_at_index (entry_index)) , IntraPackLookup :: Multi { index , required_pack_index , } => index . lookup (id) . and_then (| entry_index | { let (pack_index , pack_offset) = index . pack_id_and_pack_offset_at_index (entry_index) ; (pack_index == * required_pack_index) . then_some (pack_offset) }) , } } }
    };
}

impl_59!()