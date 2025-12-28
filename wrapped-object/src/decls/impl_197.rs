macro_rules! deps {
    () => {
        Result!();
        SectionTable!();
        ReadRef!();
        CoffFile!();
        CoffCommon!();
        SymbolTable!();
        CoffHeader!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < 'data , R : ReadRef < 'data > , Coff : CoffHeader > CoffFile < 'data , R , Coff > { # [doc = " Parse the raw COFF file data."] pub fn parse (data : R) -> Result < Self > { let mut offset = 0 ; let header = Coff :: parse (data , & mut offset) ? ; let sections = header . sections (data , offset) ? ; let symbols = header . symbols (data) ? ; Ok (CoffFile { header , common : CoffCommon { sections , symbols , image_base : 0 , } , data , }) } # [doc = " Get the raw COFF file header."] pub fn coff_header (& self) -> & 'data Coff { self . header } # [doc = " Get the COFF section table."] pub fn coff_section_table (& self) -> SectionTable < 'data > { self . common . sections } # [doc = " Get the COFF symbol table."] pub fn coff_symbol_table (& self) -> & SymbolTable < 'data , R , Coff > { & self . common . symbols } }
    };
}

impl_197!()