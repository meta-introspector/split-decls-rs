macro_rules! SYSTEMTIME {
    () => {
        # [repr (C)] # [derive (Clone , Copy , Default)] pub struct SYSTEMTIME { pub wYear : u16 , pub wMonth : u16 , pub wDayOfWeek : u16 , pub wDay : u16 , pub wHour : u16 , pub wMinute : u16 , pub wSecond : u16 , pub wMilliseconds : u16 , }
    };
}

SYSTEMTIME!()