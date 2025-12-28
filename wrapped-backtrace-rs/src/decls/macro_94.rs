macro_rules! deps {
    () => {
        PCSTR!();
        PCWSTR!();
        PSTR!();
        BOOL!();
    };
}

macro_rules! macro_94 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn WideCharToMultiByte (codepage : u32 , dwflags : u32 , lpwidecharstr : PCWSTR , cchwidechar : i32 , lpmultibytestr : PSTR , cbmultibyte : i32 , lpdefaultchar : PCSTR , lpuseddefaultchar : * mut BOOL) -> i32) ;
    };
}

macro_94!()