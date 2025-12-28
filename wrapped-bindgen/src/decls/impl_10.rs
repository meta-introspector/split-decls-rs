macro_rules! deps {
    () => {
        Type!();
        GUID!();
        Config!();
        TokenStream!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Config < '_ > { pub fn write_cpp_const_guid (& self , name : TokenStream , value : & GUID) -> TokenStream { let crate_name = self . write_core () ; let value = self . write_guid_u128 (value) ; quote ! { pub const # name : # crate_name GUID = # crate_name GUID :: from_u128 (# value) ; } } pub fn write_guid_u128 (& self , value : & GUID) -> TokenStream { format ! ("0x{:08x?}_{:04x?}_{:04x?}_{:02x?}{:02x?}_{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}" , value . 0 , value . 1 , value . 2 , value . 3 , value . 4 , value . 5 , value . 6 , value . 7 , value . 8 , value . 9 , value . 10) . into () } pub fn field_initializer < 'a > (& self , field : Field , input : & 'a str) -> (TokenStream , & 'a str) { let name = to_ident (field . name ()) ; match field . ty (None) { Type :: GUID => { let (literals , rest) = read_literal_array (input , 11) ; let value = self . write_guid_u128 (& GUID (literals [0] . parse () . unwrap () , literals [1] . parse () . unwrap () , literals [2] . parse () . unwrap () , literals [3] . parse () . unwrap () , literals [4] . parse () . unwrap () , literals [5] . parse () . unwrap () , literals [6] . parse () . unwrap () , literals [7] . parse () . unwrap () , literals [8] . parse () . unwrap () , literals [9] . parse () . unwrap () , literals [10] . parse () . unwrap () ,)) ; let crate_name = self . write_core () ; (quote ! { # name : # crate_name GUID :: from_u128 (# value) , } , rest) } Type :: ArrayFixed (_ , len) => { let (literals , rest) = read_literal_array (input , len) ; let literals = literals . iter () . map (| literal | TokenStream :: from (* literal)) ; (quote ! { # name : [# (# literals ,) *] , } , rest) } Type :: PtrMut (_ , _) => { let (_ , rest) = read_literal (input) ; (quote ! { # name : core :: ptr :: null_mut () , } , rest) } _ => { let (literal , rest) = read_literal (input) ; let literal : TokenStream = literal . into () ; (quote ! { # name : # literal , } , rest) } } } }
    };
}

impl_10!();