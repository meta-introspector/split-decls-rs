macro_rules! deps {
    () => {
        ValueParser!();
        CountType!();
        ArgAction!();
        AnyValueId!();
        OsStr!();
        ValueRange!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl ArgAction { # [doc = " Returns whether this action accepts values on the command-line"] # [doc = ""] # [doc = " [`default_values`][super::Arg::default_values] and [`env`][super::Arg::env] may still be"] # [doc = " processed."] pub fn takes_values (& self) -> bool { match self { Self :: Set => true , Self :: Append => true , Self :: SetTrue => false , Self :: SetFalse => false , Self :: Count => false , Self :: Help => false , Self :: HelpShort => false , Self :: HelpLong => false , Self :: Version => false , } } # [cfg (debug_assertions)] pub (crate) fn max_num_args (& self) -> ValueRange { match self { Self :: Set => ValueRange :: FULL , Self :: Append => ValueRange :: FULL , Self :: SetTrue => ValueRange :: OPTIONAL , Self :: SetFalse => ValueRange :: OPTIONAL , Self :: Count => ValueRange :: EMPTY , Self :: Help => ValueRange :: EMPTY , Self :: HelpShort => ValueRange :: EMPTY , Self :: HelpLong => ValueRange :: EMPTY , Self :: Version => ValueRange :: EMPTY , } } pub (crate) fn default_num_args (& self) -> ValueRange { match self { Self :: Set => ValueRange :: SINGLE , Self :: Append => ValueRange :: SINGLE , Self :: SetTrue => ValueRange :: EMPTY , Self :: SetFalse => ValueRange :: EMPTY , Self :: Count => ValueRange :: EMPTY , Self :: Help => ValueRange :: EMPTY , Self :: HelpShort => ValueRange :: EMPTY , Self :: HelpLong => ValueRange :: EMPTY , Self :: Version => ValueRange :: EMPTY , } } pub (crate) fn default_value (& self) -> Option < & 'static std :: ffi :: OsStr > { match self { Self :: Set => None , Self :: Append => None , Self :: SetTrue => Some (std :: ffi :: OsStr :: new ("false")) , Self :: SetFalse => Some (std :: ffi :: OsStr :: new ("true")) , Self :: Count => Some (std :: ffi :: OsStr :: new ("0")) , Self :: Help => None , Self :: HelpShort => None , Self :: HelpLong => None , Self :: Version => None , } } pub (crate) fn default_missing_value (& self) -> Option < & 'static std :: ffi :: OsStr > { match self { Self :: Set => None , Self :: Append => None , Self :: SetTrue => Some (std :: ffi :: OsStr :: new ("true")) , Self :: SetFalse => Some (std :: ffi :: OsStr :: new ("false")) , Self :: Count => None , Self :: Help => None , Self :: HelpShort => None , Self :: HelpLong => None , Self :: Version => None , } } pub (crate) fn default_value_parser (& self) -> Option < super :: ValueParser > { match self { Self :: Set => None , Self :: Append => None , Self :: SetTrue => Some (super :: ValueParser :: bool ()) , Self :: SetFalse => Some (super :: ValueParser :: bool ()) , Self :: Count => Some (crate :: value_parser ! (u8) . into ()) , Self :: Help => None , Self :: HelpShort => None , Self :: HelpLong => None , Self :: Version => None , } } # [cfg (debug_assertions)] pub (crate) fn value_type_id (& self) -> Option < AnyValueId > { match self { Self :: Set => None , Self :: Append => None , Self :: SetTrue => None , Self :: SetFalse => None , Self :: Count => Some (AnyValueId :: of :: < CountType > ()) , Self :: Help => None , Self :: HelpShort => None , Self :: HelpLong => None , Self :: Version => None , } } }
    };
}

impl_34!()