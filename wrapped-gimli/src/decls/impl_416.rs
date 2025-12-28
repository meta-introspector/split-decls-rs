macro_rules! deps {
    () => {
        LineInstructions!();
        LineProgramHeader!();
        LineInstruction!();
        Reader!();
        Result!();
    };
}

macro_rules! impl_416 {
    () => {
        deps!();
        impl < R : Reader > LineInstructions < R > { # [doc = " Advance the iterator and return the next instruction."] # [doc = ""] # [doc = " Returns the newly parsed instruction as `Ok(Some(instruction))`. Returns"] # [doc = " `Ok(None)` when iteration is complete and all instructions have already been"] # [doc = " parsed and yielded. If an error occurs while parsing the next attribute,"] # [doc = " then this error is returned as `Err(e)`, and all subsequent calls return"] # [doc = " `Ok(None)`."] # [doc = ""] # [doc = " Unfortunately, the `header` parameter means that this cannot be a"] # [doc = " `FallibleIterator`."] # [inline (always)] pub fn next_instruction (& mut self , header : & LineProgramHeader < R > ,) -> Result < Option < LineInstruction < R > > > { if self . input . is_empty () { return Ok (None) ; } match LineInstruction :: parse (header , & mut self . input) { Ok (instruction) => Ok (Some (instruction)) , Err (e) => { self . input . empty () ; Err (e) } } } }
    };
}

impl_416!()