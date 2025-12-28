macro_rules! deps {
    () => {
        SectionBaseAddresses!();
    };
}

macro_rules! BaseAddresses {
    () => {
        deps!();
        # [doc = " Optional base addresses for the relative `DW_EH_PE_*` encoded pointers."] # [doc = ""] # [doc = " During CIE/FDE parsing, if a relative pointer is encountered for a base"] # [doc = " address that is unknown, an Err will be returned."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::BaseAddresses;"] # [doc = ""] # [doc = " # fn foo() {"] # [doc = " # let address_of_eh_frame_hdr_section_in_memory = unimplemented!();"] # [doc = " # let address_of_eh_frame_section_in_memory = unimplemented!();"] # [doc = " # let address_of_text_section_in_memory = unimplemented!();"] # [doc = " # let address_of_got_section_in_memory = unimplemented!();"] # [doc = " # let address_of_the_start_of_current_func = unimplemented!();"] # [doc = " let bases = BaseAddresses::default()"] # [doc = "     .set_eh_frame_hdr(address_of_eh_frame_hdr_section_in_memory)"] # [doc = "     .set_eh_frame(address_of_eh_frame_section_in_memory)"] # [doc = "     .set_text(address_of_text_section_in_memory)"] # [doc = "     .set_got(address_of_got_section_in_memory);"] # [doc = " # let _ = bases;"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Default , Debug , PartialEq , Eq)] pub struct BaseAddresses { # [doc = " The base addresses to use for pointers in the `.eh_frame_hdr` section."] pub eh_frame_hdr : SectionBaseAddresses , # [doc = " The base addresses to use for pointers in the `.eh_frame` section."] pub eh_frame : SectionBaseAddresses , }
    };
}

BaseAddresses!();