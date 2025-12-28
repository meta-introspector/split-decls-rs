macro_rules! deps {
    () => {
        Reader!();
        ReaderOffset!();
        LineProgramHeader!();
    };
}

macro_rules! CompleteLineProgram {
    () => {
        deps!();
        # [doc = " A line number program that has previously been run to completion."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct CompleteLineProgram < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { header : LineProgramHeader < R , Offset > , }
    };
}

CompleteLineProgram!()