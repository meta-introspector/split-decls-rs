macro_rules! deps {
    () => {
        LineProgram!();
        ReaderOffset!();
        CompleteLineProgram!();
        Reader!();
        LineProgramHeader!();
        FileEntry!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        impl < 'program , R , Offset > LineProgram < R , Offset > for & 'program CompleteLineProgram < R , Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { fn header (& self) -> & LineProgramHeader < R , Offset > { & self . header } fn add_file (& mut self , _ : FileEntry < R , Offset >) { } }
    };
}

impl_407!()