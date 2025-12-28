macro_rules! deps {
    () => {
        AttributeSpecification!();
        Result!();
        Error!();
        UnitHeader!();
        Reader!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl AttributeSpecification { # [doc = " Construct a new `AttributeSpecification` from the given name and form"] # [doc = " and implicit const value."] # [inline] pub fn new (name : constants :: DwAt , form : constants :: DwForm , implicit_const_value : Option < i64 > ,) -> AttributeSpecification { debug_assert ! ((form == constants :: DW_FORM_implicit_const && implicit_const_value . is_some ()) || (form != constants :: DW_FORM_implicit_const && implicit_const_value . is_none ())) ; AttributeSpecification { name , form , implicit_const_value : implicit_const_value . unwrap_or (0) , } } # [doc = " Get the attribute's name."] # [inline] pub fn name (& self) -> constants :: DwAt { self . name } # [doc = " Get the attribute's form."] # [inline] pub fn form (& self) -> constants :: DwForm { self . form } # [doc = " Get the attribute's implicit const value."] # [inline] pub fn implicit_const_value (& self) -> Option < i64 > { if self . form == constants :: DW_FORM_implicit_const { Some (self . implicit_const_value) } else { None } } # [doc = " Return the size of the attribute, in bytes."] # [doc = ""] # [doc = " Note that because some attributes are variably sized, the size cannot"] # [doc = " always be known without parsing, in which case we return `None`."] pub fn size < R : Reader > (& self , header : & UnitHeader < R >) -> Option < usize > { get_attribute_size (self . form , header . encoding ()) . map (usize :: from) } # [doc = " Parse an attribute's form."] fn parse_form < R : Reader > (input : & mut R) -> Result < constants :: DwForm > { let val = input . read_uleb128_u16 () ? ; if val == 0 { Err (Error :: AttributeFormZero) } else { Ok (constants :: DwForm (val)) } } # [doc = " Parse an attribute specification. Returns `None` for the null attribute"] # [doc = " specification, `Some` for an actual attribute specification."] fn parse < R : Reader > (input : & mut R) -> Result < Option < AttributeSpecification > > { let name = input . read_uleb128_u16 () ? ; if name == 0 { let form = input . read_uleb128_u16 () ? ; return if form == 0 { Ok (None) } else { Err (Error :: ExpectedZero) } ; } let name = constants :: DwAt (name) ; let form = Self :: parse_form (input) ? ; let implicit_const_value = if form == constants :: DW_FORM_implicit_const { Some (input . read_sleb128 () ?) } else { None } ; let spec = AttributeSpecification :: new (name , form , implicit_const_value) ; Ok (Some (spec)) } }
    };
}

impl_355!();