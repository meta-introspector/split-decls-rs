macro_rules! deps {
    () => {
        Result!();
        Names!();
        Error!();
        ObjectIdentifier!();
        Database!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < 'a > Database < 'a > { # [doc = " Looks up a name for an OID."] # [doc = ""] # [doc = " Errors if the input is not a valid OID."] # [doc = " Returns the input if no name is found."] pub fn resolve < 'b > (& self , oid : & 'b str) -> Result < & 'b str , Error > where 'a : 'b , { Ok (self . by_oid (& oid . parse () ?) . unwrap_or (oid)) } # [doc = " Finds a named oid by its associated OID."] pub const fn by_oid (& self , oid : & ObjectIdentifier) -> Option < & 'a str > { let mut i = 0 ; while i < self . 0 . len () { let lhs = self . 0 [i] . 0 ; if lhs . ber . eq (& oid . ber) { return Some (self . 0 [i] . 1) ; } i += 1 ; } None } # [doc = " Finds a named oid by its associated name."] pub const fn by_name (& self , name : & str) -> Option < & 'a ObjectIdentifier > { let mut i = 0 ; while i < self . 0 . len () { let lhs = self . 0 [i] . 1 ; if eq_case (lhs . as_bytes () , name . as_bytes ()) { return Some (self . 0 [i] . 0) ; } i += 1 ; } None } # [doc = " Return the list of matched name for the OID."] pub const fn find_names_for_oid (& self , oid : ObjectIdentifier) -> Names < 'a > { Names { database : * self , oid , position : 0 , } } }
    };
}

impl_45!();