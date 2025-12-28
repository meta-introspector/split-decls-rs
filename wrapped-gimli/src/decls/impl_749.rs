macro_rules! deps {
    () => {
        Encoding!();
        LineInstruction!();
        DebugLine!();
        Result!();
        Writer!();
    };
}

macro_rules! impl_749 {
    () => {
        deps!();
        impl LineInstruction { # [doc = " Write the line number instruction to the given section."] fn write < W : Writer > (self , w : & mut DebugLine < W > , encoding : Encoding) -> Result < () > { use self :: LineInstruction :: * ; match self { Special (val) => w . write_u8 (val) ? , Copy => w . write_u8 (constants :: DW_LNS_copy . 0) ? , AdvancePc (val) => { w . write_u8 (constants :: DW_LNS_advance_pc . 0) ? ; w . write_uleb128 (val) ? ; } AdvanceLine (val) => { w . write_u8 (constants :: DW_LNS_advance_line . 0) ? ; w . write_sleb128 (val) ? ; } SetFile (val) => { w . write_u8 (constants :: DW_LNS_set_file . 0) ? ; w . write_uleb128 (val . raw (encoding . version)) ? ; } SetColumn (val) => { w . write_u8 (constants :: DW_LNS_set_column . 0) ? ; w . write_uleb128 (val) ? ; } NegateStatement => w . write_u8 (constants :: DW_LNS_negate_stmt . 0) ? , SetBasicBlock => w . write_u8 (constants :: DW_LNS_set_basic_block . 0) ? , ConstAddPc => w . write_u8 (constants :: DW_LNS_const_add_pc . 0) ? , SetPrologueEnd => w . write_u8 (constants :: DW_LNS_set_prologue_end . 0) ? , SetEpilogueBegin => w . write_u8 (constants :: DW_LNS_set_epilogue_begin . 0) ? , SetIsa (val) => { w . write_u8 (constants :: DW_LNS_set_isa . 0) ? ; w . write_uleb128 (val) ? ; } EndSequence => { w . write_u8 (0) ? ; w . write_uleb128 (1) ? ; w . write_u8 (constants :: DW_LNE_end_sequence . 0) ? ; } SetAddress (address) => { w . write_u8 (0) ? ; w . write_uleb128 (1 + u64 :: from (encoding . address_size)) ? ; w . write_u8 (constants :: DW_LNE_set_address . 0) ? ; w . write_address (address , encoding . address_size) ? ; } SetDiscriminator (val) => { let mut bytes = [0u8 ; 10] ; let len = leb128 :: write :: unsigned (& mut { & mut bytes [..] } , val) . unwrap () ; w . write_u8 (0) ? ; w . write_uleb128 (1 + len as u64) ? ; w . write_u8 (constants :: DW_LNE_set_discriminator . 0) ? ; w . write (& bytes [.. len]) ? ; } } Ok (()) } }
    };
}

impl_749!()