macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! macro_1404 {
    () => {
        deps!();
        pin_project ! { # [doc = " UnfoldState used for stream and sink unfolds"] # [project = UnfoldStateProj] # [project_replace = UnfoldStateProjReplace] # [derive (Debug)] pub (crate) enum UnfoldState < T , Fut > { Value { value : T , } , Future { # [pin] future : Fut , } , Empty , } }
    };
}

macro_1404!()