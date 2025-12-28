macro_rules! deps {
    () => {
        Type!();
        Field!();
        Constant!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < 'a > Field < 'a > { pub fn flags (& self) -> FieldAttributes { FieldAttributes (self . usize (0) . try_into () . unwrap ()) } pub fn name (& self) -> & 'a str { self . str (1) } pub fn ty (& self) -> Type { let mut blob = self . blob (2) ; let prolog = blob . read_u8 () ; debug_assert_eq ! (prolog , 0x6) ; blob . read_type_signature (& []) } pub fn constant (& self) -> Option < Constant < 'a > > { self . equal_range (1 , HasConstant :: Field (* self) . encode ()) . next () } }
    };
}

impl_89!();