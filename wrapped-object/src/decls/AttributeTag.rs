macro_rules! deps {
    () => {
        SymbolId!();
        Symbol!();
        SectionId!();
        File!();
        Section!();
    };
}

macro_rules! AttributeTag {
    () => {
        deps!();
        # [doc = " The tag for a sub-subsection in an attributes section."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum AttributeTag { # [doc = " The attributes apply to the whole file."] # [doc = ""] # [doc = " Correspeonds to [`elf::Tag_File`]."] File , # [doc = " The attributes apply to the given sections."] # [doc = ""] # [doc = " Correspeonds to [`elf::Tag_Section`]."] Section (Vec < SectionId >) , # [doc = " The attributes apply to the given symbols."] # [doc = ""] # [doc = " Correspeonds to [`elf::Tag_Symbol`]."] Symbol (Vec < SymbolId >) , }
    };
}

AttributeTag!()