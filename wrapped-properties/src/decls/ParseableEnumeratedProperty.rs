macro_rules! ParseableEnumeratedProperty {
    () => {
        # [doc = " A property whose value names can be parsed from strings."] pub trait ParseableEnumeratedProperty : crate :: private :: Sealed + TrieValue { # [doc (hidden)] type DataMarker : DataMarker < DataStruct = PropertyValueNameToEnumMap < 'static > > ; # [doc (hidden)] # [cfg (feature = "compiled_data")] const SINGLETON : & 'static PropertyValueNameToEnumMap < 'static > ; }
    };
}

ParseableEnumeratedProperty!();