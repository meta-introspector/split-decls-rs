macro_rules! deps {
    () => {
        EndianSlice!();
        DebugLine!();
        LittleEndian!();
        Result!();
        LineProgramHeader!();
        IncompleteLineProgram!();
        DebugLineOffset!();
        Reader!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        impl < R : Reader > DebugLine < R > { # [doc = " Parse the line number program whose header is at the given `offset` in the"] # [doc = " `.debug_line` section."] # [doc = ""] # [doc = " The `address_size` must match the compilation unit that the lines apply to."] # [doc = " The `comp_dir` should be from the `DW_AT_comp_dir` attribute of the compilation"] # [doc = " unit. The `comp_name` should be from the `DW_AT_name` attribute of the"] # [doc = " compilation unit."] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use gimli::{DebugLine, DebugLineOffset, IncompleteLineProgram, EndianSlice, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [];"] # [doc = " # let read_debug_line_section_somehow = || &buf;"] # [doc = " let debug_line = DebugLine::new(read_debug_line_section_somehow(), LittleEndian);"] # [doc = ""] # [doc = " // In a real example, we'd grab the offset via a compilation unit"] # [doc = " // entry's `DW_AT_stmt_list` attribute, and the address size from that"] # [doc = " // unit directly."] # [doc = " let offset = DebugLineOffset(0);"] # [doc = " let address_size = 8;"] # [doc = ""] # [doc = " let program = debug_line.program(offset, address_size, None, None)"] # [doc = "     .expect(\"should have found a header at that offset, and parsed it OK\");"] # [doc = " ```"] pub fn program (& self , offset : DebugLineOffset < R :: Offset > , address_size : u8 , comp_dir : Option < R > , comp_name : Option < R > ,) -> Result < IncompleteLineProgram < R > > { let input = & mut self . debug_line_section . clone () ; input . skip (offset . 0) ? ; let header = LineProgramHeader :: parse (input , offset , address_size , comp_dir , comp_name) ? ; let program = IncompleteLineProgram { header } ; Ok (program) } }
    };
}

impl_401!();