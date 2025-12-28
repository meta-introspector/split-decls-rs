macro_rules! deps {
    () => {
        LineInstructions!();
        Reader!();
        ReaderOffset!();
        LineProgram!();
        Section!();
        LineRow!();
    };
}

macro_rules! LineRows {
    () => {
        deps!();
        # [doc = " Executes a `LineProgram` to iterate over the rows in the matrix of line number information."] # [doc = ""] # [doc = " \"The hypothetical machine used by a consumer of the line number information"] # [doc = " to expand the byte-coded instruction stream into a matrix of line number"] # [doc = " information.\" -- Section 6.2.1"] # [derive (Debug , Clone)] pub struct LineRows < R , Program , Offset = < R as Reader > :: Offset > where Program : LineProgram < R , Offset > , R : Reader < Offset = Offset > , Offset : ReaderOffset , { program : Program , row : LineRow , instructions : LineInstructions < R > , }
    };
}

LineRows!();