macro_rules! deps {
    () => {
        AttributeSpecification!();
        Attributes!();
        Reader!();
        Abbreviation!();
        Result!();
        Error!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl Abbreviation { # [doc = " Construct a new `Abbreviation`."] # [doc = ""] # [doc = " ### Panics"] # [doc = ""] # [doc = " Panics if `code` is `0`."] pub (crate) fn new (code : u64 , tag : constants :: DwTag , has_children : constants :: DwChildren , attributes : Attributes ,) -> Abbreviation { assert_ne ! (code , 0) ; Abbreviation { code , tag , has_children , attributes , } } # [doc = " Get this abbreviation's code."] # [inline] pub fn code (& self) -> u64 { self . code } # [doc = " Get this abbreviation's tag."] # [inline] pub fn tag (& self) -> constants :: DwTag { self . tag } # [doc = " Return true if this abbreviation's type has children, false otherwise."] # [inline] pub fn has_children (& self) -> bool { self . has_children == constants :: DW_CHILDREN_yes } # [doc = " Get this abbreviation's attributes."] # [inline] pub fn attributes (& self) -> & [AttributeSpecification] { & self . attributes [..] } # [doc = " Parse an abbreviation's tag."] fn parse_tag < R : Reader > (input : & mut R) -> Result < constants :: DwTag > { let val = input . read_uleb128_u16 () ? ; if val == 0 { Err (Error :: AbbreviationTagZero) } else { Ok (constants :: DwTag (val)) } } # [doc = " Parse an abbreviation's \"does the type have children?\" byte."] fn parse_has_children < R : Reader > (input : & mut R) -> Result < constants :: DwChildren > { let val = input . read_u8 () ? ; let val = constants :: DwChildren (val) ; if val == constants :: DW_CHILDREN_no || val == constants :: DW_CHILDREN_yes { Ok (val) } else { Err (Error :: BadHasChildren) } } # [doc = " Parse a series of attribute specifications, terminated by a null attribute"] # [doc = " specification."] fn parse_attributes < R : Reader > (input : & mut R) -> Result < Attributes > { let mut attrs = Attributes :: new () ; while let Some (attr) = AttributeSpecification :: parse (input) ? { attrs . push (attr) ; } Ok (attrs) } # [doc = " Parse an abbreviation. Return `None` for the null abbreviation, `Some`"] # [doc = " for an actual abbreviation."] fn parse < R : Reader > (input : & mut R) -> Result < Option < Abbreviation > > { if input . is_empty () { return Ok (None) ; } let code = input . read_uleb128 () ? ; if code == 0 { return Ok (None) ; } let tag = Self :: parse_tag (input) ? ; let has_children = Self :: parse_has_children (input) ? ; let attributes = Self :: parse_attributes (input) ? ; let abbrev = Abbreviation :: new (code , tag , has_children , attributes) ; Ok (Some (abbrev)) } }
    };
}

impl_344!()