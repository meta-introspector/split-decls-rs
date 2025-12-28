macro_rules! deps {
    () => {
        Result!();
        SectionId!();
        RelocationTarget!();
        Writer!();
        Address!();
        Relocation!();
        Section!();
        Error!();
        RelocateWriter!();
    };
}

macro_rules! impl_698 {
    () => {
        deps!();
        impl < T : RelocateWriter > Writer for T { type Endian = < < T as RelocateWriter > :: Writer as Writer > :: Endian ; fn endian (& self) -> Self :: Endian { self . writer () . endian () } fn len (& self) -> usize { self . writer () . len () } fn write (& mut self , bytes : & [u8]) -> Result < () > { self . writer_mut () . write (bytes) } fn write_at (& mut self , offset : usize , bytes : & [u8]) -> Result < () > { self . writer_mut () . write_at (offset , bytes) } fn write_address (& mut self , address : Address , size : u8) -> Result < () > { match address { Address :: Constant (val) => self . writer_mut () . write_udata (val , size) , Address :: Symbol { symbol , addend } => { self . relocate (Relocation { offset : self . len () , size , target : RelocationTarget :: Symbol (symbol) , addend , eh_pe : None , }) ; self . writer_mut () . write_udata (0 , size) } } } fn write_offset (& mut self , val : usize , section : SectionId , size : u8) -> Result < () > { self . relocate (Relocation { offset : self . len () , size , target : RelocationTarget :: Section (section) , addend : val as i64 , eh_pe : None , }) ; self . writer_mut () . write_udata (0 , size) } fn write_offset_at (& mut self , offset : usize , val : usize , section : SectionId , size : u8 ,) -> Result < () > { self . relocate (Relocation { offset , size , target : RelocationTarget :: Section (section) , addend : val as i64 , eh_pe : None , }) ; self . writer_mut () . write_udata_at (offset , 0 , size) } fn write_eh_pointer (& mut self , address : Address , eh_pe : constants :: DwEhPe , size : u8 ,) -> Result < () > { match address { Address :: Constant (_) => self . writer_mut () . write_eh_pointer (address , eh_pe , size) , Address :: Symbol { symbol , addend } => { let size = match eh_pe . format () { constants :: DW_EH_PE_absptr => size , constants :: DW_EH_PE_udata2 => 2 , constants :: DW_EH_PE_udata4 => 4 , constants :: DW_EH_PE_udata8 => 8 , constants :: DW_EH_PE_sdata2 => 2 , constants :: DW_EH_PE_sdata4 => 4 , constants :: DW_EH_PE_sdata8 => 8 , _ => return Err (Error :: UnsupportedPointerEncoding (eh_pe)) , } ; self . relocate (Relocation { offset : self . len () , size , target : RelocationTarget :: Symbol (symbol) , addend , eh_pe : Some (eh_pe) , }) ; self . writer_mut () . write_udata (0 , size) } } } }
    };
}

impl_698!()