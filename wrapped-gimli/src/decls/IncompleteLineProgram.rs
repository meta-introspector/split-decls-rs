macro_rules! deps {
    () => {
        Reader!();
        LineProgramHeader!();
        ReaderOffset!();
    };
}

macro_rules! IncompleteLineProgram {
    () => {
        deps!();
        # [doc = " A line number program that has not been run to completion."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct IncompleteLineProgram < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { header : LineProgramHeader < R , Offset > , }
    };
}

IncompleteLineProgram!();