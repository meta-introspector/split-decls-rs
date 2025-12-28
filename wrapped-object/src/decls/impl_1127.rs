macro_rules! deps {
    () => {
        Section!();
        SectionId!();
        SectionData!();
        ByteString!();
        Sections!();
        Bytes!();
    };
}

macro_rules! impl_1127 {
    () => {
        deps!();
        impl < 'data > Sections < 'data > { # [doc = " Add a new section to the table."] pub fn add (& mut self) -> & mut Section < 'data > { let id = self . next_id () ; self . push (Section { id , delete : false , name : ByteString :: default () , sh_type : 0 , sh_flags : 0 , sh_addr : 0 , sh_offset : 0 , sh_size : 0 , sh_link_section : None , sh_info : 0 , sh_info_section : None , sh_addralign : 0 , sh_entsize : 0 , data : SectionData :: Data (Bytes :: default ()) , }) } # [doc = " Add a copy of a section to the table."] # [doc = ""] # [doc = " This will set the file offset of the copy to zero."] # [doc = " [`Segment::append_section`] can be used to assign a valid file offset and a new address."] pub fn copy (& mut self , id : SectionId) -> & mut Section < 'data > { let section = self . get (id) ; let id = self . next_id () ; let name = section . name . clone () ; let sh_type = section . sh_type ; let sh_flags = section . sh_flags ; let sh_addr = section . sh_addr ; let sh_size = section . sh_size ; let sh_link_section = section . sh_link_section ; let sh_info = section . sh_info ; let sh_info_section = section . sh_info_section ; let sh_addralign = section . sh_addralign ; let sh_entsize = section . sh_entsize ; let data = section . data . clone () ; self . push (Section { id , delete : false , name , sh_type , sh_flags , sh_addr , sh_offset : 0 , sh_size , sh_link_section , sh_info , sh_info_section , sh_addralign , sh_entsize , data , }) } }
    };
}

impl_1127!()