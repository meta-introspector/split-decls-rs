// Generated macro for EhHdrTableIter (struct)
macro_rules! Depcrate_read_cfiEhHdrTableIter {
() => {
// Module: crate::read::cfi
// Provides: {"EhHdrTableIter"}
// Dependencies: {}
# [doc = " An iterator for `.eh_frame_hdr` section's binary search table."] # [doc = ""] # [doc = " Each table entry consists of a tuple containing an  `initial_location` and `address`."] # [doc = " The `initial location` represents the first address that the targeted FDE"] # [doc = " is able to decode. The `address` is the address of the FDE in the `.eh_frame` section."] # [doc = " The `address` can be converted with `EhHdrTable::pointer_to_offset` and `EhFrame::fde_from_offset` to an FDE."] # [derive (Debug)] pub struct EhHdrTableIter < 'a , 'bases , R : Reader > { hdr : & 'a ParsedEhFrameHdr < R > , table : R , bases : & 'bases BaseAddresses , remain : u64 , }
};
}
