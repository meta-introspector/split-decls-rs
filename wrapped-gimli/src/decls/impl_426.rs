macro_rules! deps {
    () => {
        EndianSlice!();
        CompleteLineProgram!();
        ResumedLineRows!();
        ReaderOffset!();
        Reader!();
        LineRows!();
        LineProgramHeader!();
        LineSequence!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        impl < R , Offset > CompleteLineProgram < R , Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { # [doc = " Retrieve the `LineProgramHeader` for this program."] pub fn header (& self) -> & LineProgramHeader < R , Offset > { & self . header } # [doc = " Construct a new `LineRows` for executing the subset of the line"] # [doc = " number program identified by 'sequence' and  generating the line information"] # [doc = " matrix."] # [doc = ""] # [doc = " ```"] # [doc = " # fn foo() {"] # [doc = " use gimli::{IncompleteLineProgram, EndianSlice, NativeEndian};"] # [doc = ""] # [doc = " fn get_line_number_program<'a>() -> IncompleteLineProgram<EndianSlice<'a, NativeEndian>> {"] # [doc = "     // Get a line number program from some offset in a"] # [doc = "     // `.debug_line` section..."] # [doc = " #   unimplemented!()"] # [doc = " }"] # [doc = ""] # [doc = " let program = get_line_number_program();"] # [doc = " let (program, sequences) = program.sequences().unwrap();"] # [doc = " for sequence in &sequences {"] # [doc = "     let mut sm = program.resume_from(sequence);"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] pub fn resume_from < 'program > (& 'program self , sequence : & LineSequence < R > ,) -> ResumedLineRows < 'program , R , Offset > { ResumedLineRows :: resume (self , sequence) } }
    };
}

impl_426!();