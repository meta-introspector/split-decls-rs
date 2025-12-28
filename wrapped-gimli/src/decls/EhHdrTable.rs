macro_rules! deps {
    () => {
        Reader!();
        ParsedEhFrameHdr!();
    };
}

macro_rules! EhHdrTable {
    () => {
        deps!();
        # [doc = " The CFI binary search table that is an optional part of the `.eh_frame_hdr` section."] # [derive (Debug , Clone)] pub struct EhHdrTable < 'a , R : Reader > { hdr : & 'a ParsedEhFrameHdr < R > , }
    };
}

EhHdrTable!()