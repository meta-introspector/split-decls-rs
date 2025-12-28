macro_rules! CacheControl {
    () => {
        # [derive (Serialize , Deserialize , Clone , Debug , PartialEq , Eq)] # [serde (tag = "cache" , rename_all = "kebab-case")] # [non_exhaustive] pub enum CacheControl { # [doc = " Do not cache this result."] Never , # [doc = " Cache this result and use it for subsequent requests in the current Cargo invocation until the specified time."] Expires { # [serde (with = "time::serde::timestamp")] expiration : OffsetDateTime , } , # [doc = " Cache this result and use it for all subsequent requests in the current Cargo invocation."] Session , # [serde (other)] Unknown , }
    };
}

CacheControl!()