macro_rules! deps {
    () => {
        ReaderOffset!();
        Encoding!();
        FileEntryFormat!();
        Reader!();
        DebugLineOffset!();
        LineEncoding!();
        AttributeValue!();
        FileEntry!();
    };
}

macro_rules! LineProgramHeader {
    () => {
        deps!();
        # [doc = " A header for a line number program in the `.debug_line` section, as defined"] # [doc = " in section 6.2.4 of the standard."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct LineProgramHeader < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { encoding : Encoding , offset : DebugLineOffset < Offset > , unit_length : Offset , header_length : Offset , line_encoding : LineEncoding , # [doc = " \"The number assigned to the first special opcode.\""] opcode_base : u8 , # [doc = " \"This array specifies the number of LEB128 operands for each of the"] # [doc = " standard opcodes. The first element of the array corresponds to the"] # [doc = " opcode whose value is 1, and the last element corresponds to the opcode"] # [doc = " whose value is `opcode_base - 1`.\""] standard_opcode_lengths : R , # [doc = " \"A sequence of directory entry format descriptions.\""] directory_entry_format : Vec < FileEntryFormat > , # [doc = " > Entries in this sequence describe each path that was searched for"] # [doc = " > included source files in this compilation. (The paths include those"] # [doc = " > directories specified explicitly by the user for the compiler to search"] # [doc = " > and those the compiler searches without explicit direction.) Each path"] # [doc = " > entry is either a full path name or is relative to the current directory"] # [doc = " > of the compilation."] # [doc = " >"] # [doc = " > The last entry is followed by a single null byte."] include_directories : Vec < AttributeValue < R , Offset > > , # [doc = " \"A sequence of file entry format descriptions.\""] file_name_entry_format : Vec < FileEntryFormat > , # [doc = " \"Entries in this sequence describe source files that contribute to the"] # [doc = " line number information for this compilation unit or is used in other"] # [doc = " contexts.\""] file_names : Vec < FileEntry < R , Offset > > , # [doc = " The encoded line program instructions."] program_buf : R , # [doc = " The current directory of the compilation."] comp_dir : Option < R > , # [doc = " The primary source file."] comp_file : Option < FileEntry < R , Offset > > , }
    };
}

LineProgramHeader!()