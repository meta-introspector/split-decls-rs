macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! CStrLike {
    () => {
        deps!();
        # [doc = " Value which can be converted into a C string."] # [doc = ""] # [doc = " The trait is used as argument to functions which wish to accept either"] # [doc = " [`&str`] or [`&CStr`](CStr) arguments while internally need to interact with"] # [doc = " C APIs.  Accepting [`&str`] may be more convenient for users but requires"] # [doc = " conversion into [`CString`] internally which requires allocation.  With this"] # [doc = " trait, latency-conscious users may choose to prepare [`CStr`] in advance and"] # [doc = " then pass it directly without having to incur the conversion cost."] # [doc = ""] # [doc = " To use the trait, function should accept `impl CStrLike` and after baking"] # [doc = " the argument (with [`CStrLike::bake`] method) it can use it as a [`&CStr`](CStr)"] # [doc = " (since the baked result dereferences into [`CStr`])."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::ffi::{CStr, CString};"] # [doc = " use rocksdb::CStrLike;"] # [doc = ""] # [doc = " fn strlen(arg: impl CStrLike) -> std::result::Result<usize, String> {"] # [doc = "     let baked = arg.bake().map_err(|err| err.to_string())?;"] # [doc = "     Ok(unsafe { libc::strlen(baked.as_ptr()) })"] # [doc = " }"] # [doc = ""] # [doc = " const FOO: &str = \"foo\";"] # [doc = " const BAR: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b\"bar\\0\") };"] # [doc = ""] # [doc = " assert_eq!(Ok(3), strlen(FOO));"] # [doc = " assert_eq!(Ok(3), strlen(BAR));"] # [doc = " ```"] pub trait CStrLike { type Baked : std :: ops :: Deref < Target = CStr > ; type Error : std :: fmt :: Debug + std :: fmt :: Display ; # [doc = " Bakes self into value which can be freely converted into [`&CStr`](CStr)."] # [doc = ""] # [doc = " This may require allocation and may fail if `self` has invalid value."] fn bake (self) -> Result < Self :: Baked , Self :: Error > ; # [doc = " Consumers and converts value into an owned [`CString`]."] # [doc = ""] # [doc = " If `Self` is already a `CString` simply returns it; if it’s a reference"] # [doc = " to a `CString` then the value is cloned.  In other cases this may"] # [doc = " require allocation and may fail if `self` has invalid value."] fn into_c_string (self) -> Result < CString , Self :: Error > ; }
    };
}

CStrLike!()