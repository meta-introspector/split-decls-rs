macro_rules! deps {
    () => {
        U32!();
        RichHeaderEntry!();
        MaskedRichHeaderEntry!();
        Bytes!();
        ReadRef!();
        RichHeaderInfo!();
        Item!();
    };
}

macro_rules! impl_733 {
    () => {
        deps!();
        impl < 'data > RichHeaderInfo < 'data > { # [doc = " Try to locate a rich header and its entries in the current PE file."] pub fn parse < R : ReadRef < 'data > > (data : R , nt_header_offset : u64) -> Option < Self > { let data = data . read_bytes_at (0 , nt_header_offset) . map (Bytes) . ok () ? ; let end_marker_offset = memmem (data . 0 , b"Rich" , 4) ? ; let xor_key = * data . read_at :: < U32 < LE > > (end_marker_offset + 4) . ok () ? ; let masked_start_marker = U32 :: new (LE , 0x536e_6144 ^ xor_key . get (LE)) ; let start_header = [masked_start_marker , xor_key , xor_key , xor_key] ; let start_sequence = bytes_of_slice (& start_header) ; let start_marker_offset = memmem (& data . 0 [.. end_marker_offset] , start_sequence , 4) ? ; let items_offset = start_marker_offset + start_sequence . len () ; let items_len = end_marker_offset - items_offset ; let item_count = items_len / mem :: size_of :: < pe :: MaskedRichHeaderEntry > () ; let items = data . read_slice_at (items_offset , item_count) . ok () ? ; Some (RichHeaderInfo { offset : start_marker_offset , length : end_marker_offset - start_marker_offset + 8 , xor_key : xor_key . get (LE) , masked_entries : items , }) } # [doc = " Returns an iterator over the unmasked entries."] pub fn unmasked_entries (& self) -> impl Iterator < Item = RichHeaderEntry > + 'data { let xor_key = self . xor_key ; self . masked_entries . iter () . map (move | entry | RichHeaderEntry { comp_id : entry . masked_comp_id . get (LE) ^ xor_key , count : entry . masked_count . get (LE) ^ xor_key , }) } }
    };
}

impl_733!();