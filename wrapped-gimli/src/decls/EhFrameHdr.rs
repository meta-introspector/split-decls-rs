macro_rules! deps {
    () => {
        Reader!();
    };
}

macro_rules! EhFrameHdr {
    () => {
        deps!();
        # [doc = " `EhFrameHdr` contains the information about the `.eh_frame_hdr` section."] # [doc = ""] # [doc = " A pointer to the start of the `.eh_frame` data, and optionally, a binary"] # [doc = " search table of pointers to the `.eh_frame` records that are found in this section."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct EhFrameHdr < R : Reader > (R) ;
    };
}

EhFrameHdr!();