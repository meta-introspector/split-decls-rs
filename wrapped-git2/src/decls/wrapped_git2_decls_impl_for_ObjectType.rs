use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ObjectType {
    /// Convert an object type to its string representation.
    pub fn str(&self) -> &'static str {
        unsafe {
            let ptr = call!(raw::git_object_type2string(* self)) as *const _;
            let data = CStr::from_ptr(ptr).to_bytes();
            str::from_utf8(data).unwrap()
        }
    }
    /// Determine if the given git_object_t is a valid loose object type.
    pub fn is_loose(&self) -> bool {
        unsafe { call!(raw::git_object_typeisloose(* self)) == 1 }
    }
    /// Convert a raw git_object_t to an ObjectType
    pub fn from_raw(raw: raw::git_object_t) -> Option<ObjectType> {
        match raw {
            raw::GIT_OBJECT_ANY => Some(ObjectType::Any),
            raw::GIT_OBJECT_COMMIT => Some(ObjectType::Commit),
            raw::GIT_OBJECT_TREE => Some(ObjectType::Tree),
            raw::GIT_OBJECT_BLOB => Some(ObjectType::Blob),
            raw::GIT_OBJECT_TAG => Some(ObjectType::Tag),
            _ => None,
        }
    }
    /// Convert this kind into its raw representation
    pub fn raw(&self) -> raw::git_object_t {
        call::convert(self)
    }
    /// Convert a string object type representation to its object type.
    pub fn from_str(s: &str) -> Option<ObjectType> {
        let raw = unsafe {
            call!(raw::git_object_string2type(CString::new(s).unwrap()))
        };
        ObjectType::from_raw(raw)
    }
}
