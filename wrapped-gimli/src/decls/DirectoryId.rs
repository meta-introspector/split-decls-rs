macro_rules! deps {
    () => {
        LineProgram!();
    };
}

macro_rules! DirectoryId {
    () => {
        deps!();
        # [doc = " An identifier for a directory in a `LineProgram`."] # [doc = ""] # [doc = " Defaults to the working directory of the compilation unit."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct DirectoryId (usize) ;
    };
}

DirectoryId!();