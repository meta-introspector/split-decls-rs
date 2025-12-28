macro_rules! deps {
    () => {
        BaseAddresses!();
    };
}

macro_rules! SectionBaseAddresses {
    () => {
        deps!();
        # [doc = " Optional base addresses for the relative `DW_EH_PE_*` encoded pointers"] # [doc = " in a particular section."] # [doc = ""] # [doc = " See `BaseAddresses` for methods that are helpful in setting these addresses."] # [derive (Clone , Default , Debug , PartialEq , Eq)] pub struct SectionBaseAddresses { # [doc = " The address of the section containing the pointer."] pub section : Option < u64 > , # [doc = " The base address for text relative pointers."] # [doc = " This is generally the address of the `.text` section."] pub text : Option < u64 > , # [doc = " The base address for data relative pointers."] # [doc = ""] # [doc = " For pointers in the `.eh_frame_hdr` section, this is the address"] # [doc = " of the `.eh_frame_hdr` section"] # [doc = ""] # [doc = " For pointers in the `.eh_frame` section, this is generally the"] # [doc = " global pointer, such as the address of the `.got` section."] pub data : Option < u64 > , }
    };
}

SectionBaseAddresses!()