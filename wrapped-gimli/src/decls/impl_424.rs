macro_rules! deps {
    () => {
        LineProgramHeader!();
        EndianSlice!();
        LineRows!();
        CompleteLineProgram!();
        LineSequence!();
        Result!();
        IncompleteLineProgram!();
        OneShotLineRows!();
        Reader!();
        ReaderOffset!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        impl < R , Offset > IncompleteLineProgram < R , Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { # [doc = " Retrieve the `LineProgramHeader` for this program."] pub fn header (& self) -> & LineProgramHeader < R , Offset > { & self . header } # [doc = " Construct a new `LineRows` for executing this program to iterate"] # [doc = " over rows in the line information matrix."] pub fn rows (self) -> OneShotLineRows < R , Offset > { OneShotLineRows :: new (self) } # [doc = " Execute the line number program, completing the `IncompleteLineProgram`"] # [doc = " into a `CompleteLineProgram` and producing an array of sequences within"] # [doc = " the line number program that can later be used with"] # [doc = " `CompleteLineProgram::resume_from`."] # [doc = ""] # [doc = " ```"] # [doc = " # fn foo() {"] # [doc = " use gimli::{IncompleteLineProgram, EndianSlice, NativeEndian};"] # [doc = ""] # [doc = " fn get_line_number_program<'a>() -> IncompleteLineProgram<EndianSlice<'a, NativeEndian>> {"] # [doc = "     // Get a line number program from some offset in a"] # [doc = "     // `.debug_line` section..."] # [doc = " #   unimplemented!()"] # [doc = " }"] # [doc = ""] # [doc = " let program = get_line_number_program();"] # [doc = " let (program, sequences) = program.sequences().unwrap();"] # [doc = " println!(\"There are {} sequences in this line number program\", sequences.len());"] # [doc = " # }"] # [doc = " ```"] # [allow (clippy :: type_complexity)] pub fn sequences (self) -> Result < (CompleteLineProgram < R , Offset > , Vec < LineSequence < R > >) > { let mut sequences = Vec :: new () ; let mut rows = self . rows () ; let mut instructions = rows . instructions . clone () ; let mut sequence_start_addr = None ; loop { let sequence_end_addr ; if rows . next_row () ? . is_none () { break ; } let row = & rows . row ; if row . end_sequence () { sequence_end_addr = row . address () ; } else if sequence_start_addr . is_none () { sequence_start_addr = Some (row . address ()) ; continue ; } else { continue ; } sequences . push (LineSequence { start : sequence_start_addr . unwrap_or (0) , end : sequence_end_addr , instructions : instructions . remove_trailing (& rows . instructions) ? , }) ; sequence_start_addr = None ; instructions = rows . instructions . clone () ; } let program = CompleteLineProgram { header : rows . program . header , } ; Ok ((program , sequences)) } }
    };
}

impl_424!()