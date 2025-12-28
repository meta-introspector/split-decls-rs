macro_rules! MacroKinds {
    () => {
        # [doc = " A set of macro kinds, for macros that can have more than one kind"] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Encodable , Decodable , Hash , Debug)] # [derive (HashStable_Generic)] pub struct MacroKinds (u8) ;
    };
}

MacroKinds!()