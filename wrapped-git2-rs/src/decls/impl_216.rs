macro_rules! deps {
    () => {
        Blob!();
        Object!();
        Binding!();
        Oid!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl < 'repo > Blob < 'repo > { # [doc = " Get the id (SHA1) of a repository blob"] pub fn id (& self) -> Oid { unsafe { Binding :: from_raw (raw :: git_blob_id (& * self . raw)) } } # [doc = " Determine if the blob content is most certainly binary or not."] pub fn is_binary (& self) -> bool { unsafe { raw :: git_blob_is_binary (& * self . raw) == 1 } } # [doc = " Get the content of this blob."] pub fn content (& self) -> & [u8] { unsafe { let data = raw :: git_blob_rawcontent (& * self . raw) as * const u8 ; let len = raw :: git_blob_rawsize (& * self . raw) as usize ; slice :: from_raw_parts (data , len) } } # [doc = " Get the size in bytes of the contents of this blob."] pub fn size (& self) -> usize { unsafe { raw :: git_blob_rawsize (& * self . raw) as usize } } # [doc = " Casts this Blob to be usable as an `Object`"] pub fn as_object (& self) -> & Object < 'repo > { unsafe { & * (self as * const _ as * const Object < 'repo >) } } # [doc = " Consumes Blob to be returned as an `Object`"] pub fn into_object (self) -> Object < 'repo > { assert_eq ! (mem :: size_of_val (& self) , mem :: size_of ::< Object <'_ >> ()) ; unsafe { mem :: transmute (self) } } }
    };
}

impl_216!();