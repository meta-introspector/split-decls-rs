macro_rules! deps {
    () => {
        CallFrameInstruction!();
        CallFrameInstructionIter!();
        Reader!();
        Result!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl < 'a , R : Reader > CallFrameInstructionIter < 'a , R > { # [doc = " Parse the next call frame instruction."] pub fn next (& mut self) -> Result < Option < CallFrameInstruction < R :: Offset > > > { if self . input . is_empty () { return Ok (None) ; } match CallFrameInstruction :: parse (& mut self . input , self . address_encoding , & self . parameters , self . vendor ,) { Ok (instruction) => Ok (Some (instruction)) , Err (e) => { self . input . empty () ; Err (e) } } } }
    };
}

impl_246!();