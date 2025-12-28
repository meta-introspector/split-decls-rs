macro_rules! deps {
    () => {
        CallFrameInstruction!();
        DebugFrame!();
        CommonInformationEntry!();
        Address!();
        SectionId!();
        Writer!();
        FrameDescriptionEntry!();
        Result!();
    };
}

macro_rules! impl_727 {
    () => {
        deps!();
        impl FrameDescriptionEntry { # [doc = " Create a new frame description entry for a function."] pub fn new (address : Address , length : u32) -> Self { FrameDescriptionEntry { address , length , lsda : None , instructions : Vec :: new () , } } # [doc = " Add an instruction."] # [doc = ""] # [doc = " Instructions must be added in increasing order of offset, or writing will fail."] pub fn add_instruction (& mut self , offset : u32 , instruction : CallFrameInstruction) { debug_assert ! (self . instructions . last () . map (| x | x . 0) . unwrap_or (0) <= offset) ; self . instructions . push ((offset , instruction)) ; } fn write < W : Writer > (& self , w : & mut W , eh_frame : bool , cie_offset : usize , cie : & CommonInformationEntry ,) -> Result < () > { let encoding = cie . encoding ; let length_offset = w . write_initial_length (encoding . format) ? ; let length_base = w . len () ; if eh_frame { w . write_udata ((w . len () - cie_offset) as u64 , 4) ? ; } else { w . write_offset (cie_offset , SectionId :: DebugFrame , encoding . format . word_size () ,) ? ; } if cie . fde_address_encoding != constants :: DW_EH_PE_absptr { w . write_eh_pointer (self . address , cie . fde_address_encoding , encoding . address_size ,) ? ; w . write_eh_pointer_data (self . length . into () , cie . fde_address_encoding . format () , encoding . address_size ,) ? ; } else { w . write_address (self . address , encoding . address_size) ? ; w . write_udata (self . length . into () , encoding . address_size) ? ; } if cie . has_augmentation () { let augmentation_length_offset = w . len () ; w . write_u8 (0) ? ; let augmentation_length_base = w . len () ; debug_assert_eq ! (self . lsda . is_some () , cie . lsda_encoding . is_some ()) ; if let (Some (lsda) , Some (lsda_encoding)) = (self . lsda , cie . lsda_encoding) { w . write_eh_pointer (lsda , lsda_encoding , encoding . address_size) ? ; } let augmentation_length = (w . len () - augmentation_length_base) as u64 ; debug_assert ! (augmentation_length < 0x80) ; w . write_udata_at (augmentation_length_offset , augmentation_length , 1) ? ; } let mut prev_offset = 0 ; for (offset , instruction) in & self . instructions { write_advance_loc (w , cie . code_alignment_factor , prev_offset , * offset) ? ; prev_offset = * offset ; instruction . write (w , encoding , cie) ? ; } write_nop (w , encoding . format . word_size () as usize + w . len () - length_base , encoding . address_size ,) ? ; let length = (w . len () - length_base) as u64 ; w . write_initial_length_at (length_offset , length , encoding . format) ? ; Ok (()) } }
    };
}

impl_727!()