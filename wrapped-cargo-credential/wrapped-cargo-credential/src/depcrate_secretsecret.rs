// Generated macro for Secret (struct)
macro_rules! Depcrate_secretSecret {
() => {
// Module: crate::secret
// Provides: {"Secret"}
// Dependencies: {}
# [doc = " A wrapper for values that should not be printed."] # [doc = ""] # [doc = " This type does not implement `Display`, and has a `Debug` impl that hides"] # [doc = " the contained value."] # [doc = ""] # [doc = " ```"] # [doc = " # use cargo_credential::Secret;"] # [doc = " let token = Secret::from(\"super secret string\");"] # [doc = " assert_eq!(format!(\"{:?}\", token), \"Secret { inner: \\\"REDACTED\\\" }\");"] # [doc = " ```"] # [doc = ""] # [doc = " Currently, we write a borrowed `Secret<T>` as `Secret<&T>`."] # [doc = " The [`as_deref`](Secret::as_deref) and [`to_owned`](Secret::to_owned) methods can"] # [doc = " be used to convert back and forth between `Secret<String>` and `Secret<&str>`."] # [derive (Default , Clone , PartialEq , Eq , Serialize , Deserialize)] # [serde (transparent)] pub struct Secret < T > { inner : T , }
};
}
