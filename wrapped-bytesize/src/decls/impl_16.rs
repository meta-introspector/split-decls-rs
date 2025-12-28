macro_rules! deps {
    () => {
        Unit!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Unit { fn factor (& self) -> u64 { match self { Self :: Byte => 1 , Self :: KiloByte => crate :: KB , Self :: MegaByte => crate :: MB , Self :: GigaByte => crate :: GB , Self :: TeraByte => crate :: TB , Self :: PetaByte => crate :: PB , Self :: ExaByte => crate :: EB , Self :: KibiByte => crate :: KIB , Self :: MebiByte => crate :: MIB , Self :: GibiByte => crate :: GIB , Self :: TebiByte => crate :: TIB , Self :: PebiByte => crate :: PIB , Self :: ExbiByte => crate :: EIB , } } }
    };
}

impl_16!()