macro_rules! GUID {
    () => {
        # [doc = " A globally unique identifier ([GUID](https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid))"] # [doc = " used to identify COM and WinRT interfaces."] # [repr (C)] # [derive (Clone , Copy , Default , PartialEq , Eq , Hash)] pub struct GUID { # [doc = " Specifies the first 8 hexadecimal digits."] pub data1 : u32 , # [doc = " Specifies the first group of 4 hexadecimal digits."] pub data2 : u16 , # [doc = " Specifies the second group of 4 hexadecimal digits."] pub data3 : u16 , # [doc = " The first 2 bytes contain the third group of 4 hexadecimal digits. The remaining 6 bytes contain the final 12 hexadecimal digits."] pub data4 : [u8 ; 8] , }
    };
}

GUID!()