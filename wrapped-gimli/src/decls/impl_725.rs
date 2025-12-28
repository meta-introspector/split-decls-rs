macro_rules! deps {
    () => {
        CallFrameInstruction!();
        CommonInformationEntry!();
        Encoding!();
        Result!();
        Writer!();
        Error!();
        Format!();
        Register!();
    };
}

macro_rules! impl_725 {
    () => {
        deps!();
        impl CommonInformationEntry { # [doc = " Create a new common information entry."] # [doc = ""] # [doc = " The encoding version must be a CFI version, not a DWARF version."] pub fn new (encoding : Encoding , code_alignment_factor : u8 , data_alignment_factor : i8 , return_address_register : Register ,) -> Self { CommonInformationEntry { encoding , code_alignment_factor , data_alignment_factor , return_address_register , personality : None , lsda_encoding : None , fde_address_encoding : constants :: DW_EH_PE_absptr , signal_trampoline : false , instructions : Vec :: new () , } } # [doc = " Add an initial instruction."] pub fn add_instruction (& mut self , instruction : CallFrameInstruction) { self . instructions . push (instruction) ; } fn has_augmentation (& self) -> bool { self . personality . is_some () || self . lsda_encoding . is_some () || self . signal_trampoline || self . fde_address_encoding != constants :: DW_EH_PE_absptr } # [doc = " Returns the section offset of the CIE."] fn write < W : Writer > (& self , w : & mut W , eh_frame : bool) -> Result < usize > { let encoding = self . encoding ; let offset = w . len () ; let length_offset = w . write_initial_length (encoding . format) ? ; let length_base = w . len () ; if eh_frame { w . write_u32 (0) ? ; } else { match encoding . format { Format :: Dwarf32 => w . write_u32 (0xffff_ffff) ? , Format :: Dwarf64 => w . write_u64 (0xffff_ffff_ffff_ffff) ? , } } if eh_frame { if encoding . version != 1 { return Err (Error :: UnsupportedVersion (encoding . version)) ; } ; } else { match encoding . version { 1 | 3 | 4 => { } _ => return Err (Error :: UnsupportedVersion (encoding . version)) , } ; } w . write_u8 (encoding . version as u8) ? ; let augmentation = self . has_augmentation () ; if augmentation { w . write_u8 (b'z') ? ; if self . lsda_encoding . is_some () { w . write_u8 (b'L') ? ; } if self . personality . is_some () { w . write_u8 (b'P') ? ; } if self . fde_address_encoding != constants :: DW_EH_PE_absptr { w . write_u8 (b'R') ? ; } if self . signal_trampoline { w . write_u8 (b'S') ? ; } } w . write_u8 (0) ? ; if encoding . version >= 4 { w . write_u8 (encoding . address_size) ? ; w . write_u8 (0) ? ; } w . write_uleb128 (self . code_alignment_factor . into ()) ? ; w . write_sleb128 (self . data_alignment_factor . into ()) ? ; if ! eh_frame && encoding . version == 1 { let register = self . return_address_register . 0 as u8 ; if u16 :: from (register) != self . return_address_register . 0 { return Err (Error :: ValueTooLarge) ; } w . write_u8 (register) ? ; } else { w . write_uleb128 (self . return_address_register . 0 . into ()) ? ; } if augmentation { let augmentation_length_offset = w . len () ; w . write_u8 (0) ? ; let augmentation_length_base = w . len () ; if let Some (eh_pe) = self . lsda_encoding { w . write_u8 (eh_pe . 0) ? ; } if let Some ((eh_pe , address)) = self . personality { w . write_u8 (eh_pe . 0) ? ; w . write_eh_pointer (address , eh_pe , encoding . address_size) ? ; } if self . fde_address_encoding != constants :: DW_EH_PE_absptr { w . write_u8 (self . fde_address_encoding . 0) ? ; } let augmentation_length = (w . len () - augmentation_length_base) as u64 ; debug_assert ! (augmentation_length < 0x80) ; w . write_udata_at (augmentation_length_offset , augmentation_length , 1) ? ; } for instruction in & self . instructions { instruction . write (w , encoding , self) ? ; } write_nop (w , encoding . format . word_size () as usize + w . len () - length_base , encoding . address_size ,) ? ; let length = (w . len () - length_base) as u64 ; w . write_initial_length_at (length_offset , length , encoding . format) ? ; Ok (offset) } }
    };
}

impl_725!();