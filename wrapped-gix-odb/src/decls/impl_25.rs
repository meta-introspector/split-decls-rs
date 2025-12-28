macro_rules! deps {
    () => {
        Handle!();
        Store!();
        AllObjects!();
        Error!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < S > super :: Handle < S > where S : Deref < Target = super :: Store > + Clone , { # [doc = " Return an iterator over all, _possibly duplicate_, objects, first the ones in all packs of all linked databases (via alternates),"] # [doc = " followed by all loose objects."] pub fn iter (& self) -> Result < AllObjects , dynamic :: load_index :: Error > { AllObjects :: new (self . store_ref ()) } }
    };
}

impl_25!();