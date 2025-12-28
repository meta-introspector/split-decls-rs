macro_rules! deps {
    () => {
        ReaderOffset!();
        LineProgramHeader!();
        LineInstruction!();
        Reader!();
        FileEntry!();
        Result!();
    };
}

macro_rules! impl_413 {
    () => {
        deps!();
        impl < R , Offset > LineInstruction < R , Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { fn parse < 'header > (header : & 'header LineProgramHeader < R > , input : & mut R ,) -> Result < LineInstruction < R > > where R : 'header , { let opcode = input . read_u8 () ? ; if opcode == 0 { let length = input . read_uleb128 () . and_then (R :: Offset :: from_u64) ? ; let mut instr_rest = input . split (length) ? ; let opcode = instr_rest . read_u8 () ? ; match constants :: DwLne (opcode) { constants :: DW_LNE_end_sequence => Ok (LineInstruction :: EndSequence) , constants :: DW_LNE_set_address => { let address = instr_rest . read_address (header . address_size ()) ? ; Ok (LineInstruction :: SetAddress (address)) } constants :: DW_LNE_define_file => { if header . version () <= 4 { let path_name = instr_rest . read_null_terminated_slice () ? ; let entry = FileEntry :: parse (& mut instr_rest , path_name) ? ; Ok (LineInstruction :: DefineFile (entry)) } else { Ok (LineInstruction :: UnknownExtended (constants :: DW_LNE_define_file , instr_rest ,)) } } constants :: DW_LNE_set_discriminator => { let discriminator = instr_rest . read_uleb128 () ? ; Ok (LineInstruction :: SetDiscriminator (discriminator)) } otherwise => Ok (LineInstruction :: UnknownExtended (otherwise , instr_rest)) , } } else if opcode >= header . opcode_base { Ok (LineInstruction :: Special (opcode)) } else { match constants :: DwLns (opcode) { constants :: DW_LNS_copy => Ok (LineInstruction :: Copy) , constants :: DW_LNS_advance_pc => { let advance = input . read_uleb128 () ? ; Ok (LineInstruction :: AdvancePc (advance)) } constants :: DW_LNS_advance_line => { let increment = input . read_sleb128 () ? ; Ok (LineInstruction :: AdvanceLine (increment)) } constants :: DW_LNS_set_file => { let file = input . read_uleb128 () ? ; Ok (LineInstruction :: SetFile (file)) } constants :: DW_LNS_set_column => { let column = input . read_uleb128 () ? ; Ok (LineInstruction :: SetColumn (column)) } constants :: DW_LNS_negate_stmt => Ok (LineInstruction :: NegateStatement) , constants :: DW_LNS_set_basic_block => Ok (LineInstruction :: SetBasicBlock) , constants :: DW_LNS_const_add_pc => Ok (LineInstruction :: ConstAddPc) , constants :: DW_LNS_fixed_advance_pc => { let advance = input . read_u16 () ? ; Ok (LineInstruction :: FixedAddPc (advance)) } constants :: DW_LNS_set_prologue_end => Ok (LineInstruction :: SetPrologueEnd) , constants :: DW_LNS_set_epilogue_begin => Ok (LineInstruction :: SetEpilogueBegin) , constants :: DW_LNS_set_isa => { let isa = input . read_uleb128 () ? ; Ok (LineInstruction :: SetIsa (isa)) } otherwise => { let mut opcode_lengths = header . standard_opcode_lengths () . clone () ; opcode_lengths . skip (R :: Offset :: from_u8 (opcode - 1)) ? ; let num_args = opcode_lengths . read_u8 () ? as usize ; match num_args { 0 => Ok (LineInstruction :: UnknownStandard0 (otherwise)) , 1 => { let arg = input . read_uleb128 () ? ; Ok (LineInstruction :: UnknownStandard1 (otherwise , arg)) } _ => { let mut args = input . clone () ; for _ in 0 .. num_args { input . read_uleb128 () ? ; } let len = input . offset_from (& args) ; args . truncate (len) ? ; Ok (LineInstruction :: UnknownStandardN (otherwise , args)) } } } } } } }
    };
}

impl_413!();