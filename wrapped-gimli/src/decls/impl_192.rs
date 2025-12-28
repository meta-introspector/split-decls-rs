macro_rules! deps {
    () => {
        BaseAddresses!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl BaseAddresses { # [doc = " Set the `.eh_frame_hdr` section base address."] # [inline] pub fn set_eh_frame_hdr (mut self , addr : u64) -> Self { self . eh_frame_hdr . section = Some (addr) ; self . eh_frame_hdr . data = Some (addr) ; self } # [doc = " Set the `.eh_frame` section base address."] # [inline] pub fn set_eh_frame (mut self , addr : u64) -> Self { self . eh_frame . section = Some (addr) ; self } # [doc = " Set the `.text` section base address."] # [inline] pub fn set_text (mut self , addr : u64) -> Self { self . eh_frame_hdr . text = Some (addr) ; self . eh_frame . text = Some (addr) ; self } # [doc = " Set the `.got` section base address."] # [inline] pub fn set_got (mut self , addr : u64) -> Self { self . eh_frame . data = Some (addr) ; self } }
    };
}

impl_192!()