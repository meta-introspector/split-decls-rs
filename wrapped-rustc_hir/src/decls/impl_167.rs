macro_rules! deps {
    () => {
        Attribute!();
        AttributeMap!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < 'tcx > AttributeMap < 'tcx > { pub const EMPTY : & 'static AttributeMap < 'static > = & AttributeMap { map : SortedMap :: new () , opt_hash : Some (Fingerprint :: ZERO) , define_opaque : None , } ; # [inline] pub fn get (& self , id : ItemLocalId) -> & 'tcx [Attribute] { self . map . get (& id) . copied () . unwrap_or (& []) } }
    };
}

impl_167!();