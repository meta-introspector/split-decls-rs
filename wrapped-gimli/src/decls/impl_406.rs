macro_rules! deps {
    () => {
        ReaderOffset!();
        FileEntry!();
        LineProgram!();
        IncompleteLineProgram!();
        Reader!();
        LineProgramHeader!();
    };
}

macro_rules! impl_406 {
    () => {
        deps!();
        impl < R , Offset > LineProgram < R , Offset > for IncompleteLineProgram < R , Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { fn header (& self) -> & LineProgramHeader < R , Offset > { & self . header } fn add_file (& mut self , file : FileEntry < R , Offset >) { self . header . file_names . push (file) ; } }
    };
}

impl_406!()