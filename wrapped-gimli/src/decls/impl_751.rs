macro_rules! deps {
    () => {
        Result!();
        Writer!();
        SectionId!();
        Encoding!();
        DebugLine!();
        DebugLineStr!();
        LineString!();
        DebugStr!();
        Error!();
    };
}

macro_rules! impl_751 {
    () => {
        deps!();
        impl LineString { # [doc = " Create a `LineString` using the normal form for the given encoding."] pub fn new < T > (val : T , encoding : Encoding , line_strings : & mut LineStringTable) -> Self where T : Into < Vec < u8 > > , { let val = val . into () ; if encoding . version <= 4 { LineString :: String (val) } else { LineString :: LineStringRef (line_strings . add (val)) } } # [doc = " Get a reference to the string data."] pub fn get < 'a > (& 'a self , strings : & 'a StringTable , line_strings : & 'a LineStringTable ,) -> & 'a [u8] { match self { LineString :: String (val) => val , LineString :: StringRef (val) => strings . get (* val) , LineString :: LineStringRef (val) => line_strings . get (* val) , } } fn form (& self) -> constants :: DwForm { match * self { LineString :: String (..) => constants :: DW_FORM_string , LineString :: StringRef (..) => constants :: DW_FORM_strp , LineString :: LineStringRef (..) => constants :: DW_FORM_line_strp , } } fn write < W : Writer > (& self , w : & mut DebugLine < W > , form : constants :: DwForm , encoding : Encoding , line_strings : & LineStringTable , strings : & StringTable ,) -> Result < () > { if form != self . form () { return Err (Error :: LineStringFormMismatch) ; } match * self { LineString :: String (ref val) => { if encoding . version <= 4 { debug_assert ! (! val . is_empty ()) ; } w . write (val) ? ; w . write_u8 (0) ? ; } LineString :: StringRef (val) => { if encoding . version < 5 { return Err (Error :: NeedVersion (5)) ; } w . write_offset (strings . offset (val) . 0 , SectionId :: DebugStr , encoding . format . word_size () ,) ? ; } LineString :: LineStringRef (val) => { if encoding . version < 5 { return Err (Error :: NeedVersion (5)) ; } w . write_offset (line_strings . offset (val) . 0 , SectionId :: DebugLineStr , encoding . format . word_size () ,) ? ; } } Ok (()) } }
    };
}

impl_751!()