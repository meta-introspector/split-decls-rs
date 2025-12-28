macro_rules! deps {
    () => {
        PropertyParser!();
        ParseableEnumeratedProperty!();
        PropertyNamesShort!();
        PropertyEnumToValueNameLookup!();
        PropertyNamesLong!();
    };
}

macro_rules! NamedEnumeratedProperty {
    () => {
        deps!();
        # [doc = " A property whose value names can be represented as strings."] pub trait NamedEnumeratedProperty : ParseableEnumeratedProperty { # [doc (hidden)] type DataStructLong : 'static + for < 'a > Yokeable < 'a , Output = Self :: DataStructLongBorrowed < 'a > > + PropertyEnumToValueNameLookup ; # [doc (hidden)] type DataStructShort : 'static + for < 'a > Yokeable < 'a , Output = Self :: DataStructShortBorrowed < 'a > > + PropertyEnumToValueNameLookup ; # [doc (hidden)] type DataStructLongBorrowed < 'a > : PropertyEnumToValueNameLookup ; # [doc (hidden)] type DataStructShortBorrowed < 'a > : PropertyEnumToValueNameLookup ; # [doc (hidden)] type DataMarkerLong : DataMarker < DataStruct = Self :: DataStructLong > ; # [doc (hidden)] type DataMarkerShort : DataMarker < DataStruct = Self :: DataStructShort > ; # [doc (hidden)] # [cfg (feature = "compiled_data")] const SINGLETON_LONG : & 'static Self :: DataStructLongBorrowed < 'static > ; # [doc (hidden)] # [cfg (feature = "compiled_data")] const SINGLETON_SHORT : & 'static Self :: DataStructShortBorrowed < 'static > ; # [doc (hidden)] fn nep_long_identity < 'a > (stat : & 'a < Self :: DataStructLong as Yokeable < 'a > > :: Output ,) -> & 'a Self :: DataStructLongBorrowed < 'a > ; # [doc (hidden)] fn nep_long_identity_static (stat : & 'static Self :: DataStructLongBorrowed < 'static > ,) -> & 'static Self :: DataStructLong ; # [doc (hidden)] fn nep_short_identity < 'a > (stat : & 'a < Self :: DataStructShort as Yokeable < 'a > > :: Output ,) -> & 'a Self :: DataStructShortBorrowed < 'a > ; # [doc (hidden)] fn nep_short_identity_static (stat : & 'static Self :: DataStructShortBorrowed < 'static > ,) -> & 'static Self :: DataStructShort ; # [doc = " Convenience method for `PropertyParser::new().get_loose(s)`"] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [cfg (feature = "compiled_data")] fn try_from_str (s : & str) -> Option < Self > { PropertyParser :: new () . get_loose (s) } # [doc = " Convenience method for `PropertyNamesLong::new().get(*self).unwrap()`"] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [cfg (feature = "compiled_data")] fn long_name (& self) -> & 'static str { PropertyNamesLong :: new () . get (* self) . unwrap_or ("unreachable") } # [doc = " Convenience method for `PropertyNamesShort::new().get(*self).unwrap()`"] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [cfg (feature = "compiled_data")] fn short_name (& self) -> & 'static str { PropertyNamesShort :: new () . get (* self) . unwrap_or ("unreachable") } }
    };
}

NamedEnumeratedProperty!();