macro_rules! deps {
    () => {
        OneShotLineRows!();
        LineSequence!();
        ReaderOffset!();
        LineRow!();
        CompleteLineProgram!();
        LineProgram!();
        Result!();
        LineRows!();
        IncompleteLineProgram!();
        LineInstructions!();
        Reader!();
        ResumedLineRows!();
        LineProgramHeader!();
    };
}

macro_rules! impl_411 {
    () => {
        deps!();
        impl < R , Program , Offset > LineRows < R , Program , Offset > where Program : LineProgram < R , Offset > , R : Reader < Offset = Offset > , Offset : ReaderOffset , { fn new (program : IncompleteLineProgram < R , Offset >) -> OneShotLineRows < R , Offset > { let row = LineRow :: new (program . header ()) ; let instructions = LineInstructions { input : program . header () . program_buf . clone () , } ; LineRows { program , row , instructions , } } fn resume < 'program > (program : & 'program CompleteLineProgram < R , Offset > , sequence : & LineSequence < R > ,) -> ResumedLineRows < 'program , R , Offset > { let row = LineRow :: new (program . header ()) ; let instructions = sequence . instructions . clone () ; LineRows { program , row , instructions , } } # [doc = " Get a reference to the header for this state machine's line number"] # [doc = " program."] # [inline] pub fn header (& self) -> & LineProgramHeader < R , Offset > { self . program . header () } # [doc = " Parse and execute the next instructions in the line number program until"] # [doc = " another row in the line number matrix is computed."] # [doc = ""] # [doc = " The freshly computed row is returned as `Ok(Some((header, row)))`."] # [doc = " If the matrix is complete, and there are no more new rows in the line"] # [doc = " number matrix, then `Ok(None)` is returned. If there was an error parsing"] # [doc = " an instruction, then `Err(e)` is returned."] # [doc = ""] # [doc = " Unfortunately, the references mean that this cannot be a"] # [doc = " `FallibleIterator`."] pub fn next_row (& mut self) -> Result < Option < (& LineProgramHeader < R , Offset > , & LineRow) > > { self . row . reset (self . program . header ()) ; loop { match self . instructions . next_instruction (self . program . header ()) { Err (err) => return Err (err) , Ok (None) => return Ok (None) , Ok (Some (instruction)) => { if self . row . execute (instruction , & mut self . program) ? { if self . row . tombstone { self . row . reset (self . program . header ()) ; } else { return Ok (Some ((self . header () , & self . row))) ; } } } } } } }
    };
}

impl_411!()