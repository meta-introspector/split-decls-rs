macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! LogLevel {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [repr (i32)] pub enum LogLevel { Debug = 0 , Info , Warn , Error , Fatal , Header , }
    };
}

LogLevel!()