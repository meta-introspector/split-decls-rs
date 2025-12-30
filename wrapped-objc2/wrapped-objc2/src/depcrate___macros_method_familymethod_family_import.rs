// Generated macro for method_family_import (module)
macro_rules! Depcrate___macros_method_familymethod_family_import {
() => {
// Module: crate::__macros::method_family
// Provides: {"method_family_import"}
// Dependencies: {}
# [doc = " Helper module where `#[unsafe(method_family = $family:ident)]` will import"] # [doc = " its value from."] # [allow (non_camel_case_types)] pub mod method_family_import { pub use super :: { AllocFamily as alloc , CopyFamily as copy , InitFamily as init , MutableCopyFamily as mutableCopy , NewFamily as new , NoneFamily as none , } ; }
};
}
