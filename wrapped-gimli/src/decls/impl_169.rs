macro_rules! deps {
    () => {
        ParsedEhFrameHdr!();
        Pointer!();
        Reader!();
        EhHdrTable!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < R : Reader > ParsedEhFrameHdr < R > { # [doc = " Returns the address of the binary's `.eh_frame` section."] pub fn eh_frame_ptr (& self) -> Pointer { self . eh_frame_ptr } # [doc = " Retrieves the CFI binary search table, if there is one."] pub fn table (& self) -> Option < EhHdrTable < '_ , R > > { if self . fde_count == 0 { None } else { Some (EhHdrTable { hdr : self }) } } }
    };
}

impl_169!();