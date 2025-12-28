macro_rules! deps {
    () => {
        OdbObject!();
        ObjectType!();
        Oid!();
    };
}

macro_rules! impl_495 {
    () => {
        deps!();
        impl < 'a > OdbObject < 'a > { # [doc = " Get the object type."] pub fn kind (& self) -> ObjectType { unsafe { ObjectType :: from_raw (raw :: git_odb_object_type (self . raw)) . unwrap () } } # [doc = " Get the object size."] pub fn len (& self) -> usize { unsafe { raw :: git_odb_object_size (self . raw) } } # [doc = " Get the object data."] pub fn data (& self) -> & [u8] { unsafe { let size = self . len () ; let ptr : * const u8 = raw :: git_odb_object_data (self . raw) as * const u8 ; let buffer = slice :: from_raw_parts (ptr , size) ; return buffer ; } } # [doc = " Get the object id."] pub fn id (& self) -> Oid { unsafe { Oid :: from_raw (raw :: git_odb_object_id (self . raw)) } } }
    };
}

impl_495!()