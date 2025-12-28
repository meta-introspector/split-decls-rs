macro_rules! deps {
    () => {
        Reader!();
        Pointer!();
    };
}

macro_rules! ParsedEhFrameHdr {
    () => {
        deps!();
        # [doc = " `ParsedEhFrameHdr` contains the parsed information from the `.eh_frame_hdr` section."] # [derive (Clone , Debug)] pub struct ParsedEhFrameHdr < R : Reader > { address_size : u8 , section : R , eh_frame_ptr : Pointer , fde_count : u64 , table_enc : DwEhPe , table : R , }
    };
}

ParsedEhFrameHdr!();