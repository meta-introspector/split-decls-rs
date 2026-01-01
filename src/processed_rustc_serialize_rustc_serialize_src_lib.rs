/* FP:lib.rs-0001 */ // Support code for encoding and decoding types.
/* FP:lib.rs-0002 */ 
/* FP:lib.rs-0003 */ // tidy-alphabetical-start
/* FP:lib.rs-0004 */ #[allow(internal_features)]
/* FP:lib.rs-0005 */ #[allow(rustc::internal)]
/* FP:lib.rs-0006 */ #[cfg_attr(test, feature(test))]
/* FP:lib.rs-0007 */ #[doc(
/* FP:lib.rs-0008 */     html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/",
/* FP:lib.rs-0009 */     html_playground_url = "https://play.rust-lang.org/",
/* FP:lib.rs-0010 */     test(attr(allow(unused_variables), deny(warnings)))
/* FP:lib.rs-0011 */ )]
/* FP:lib.rs-0012 */ #[doc(rust_logo)]
/* FP:lib.rs-0013 */ #[feature(core_intrinsics)]
/* FP:lib.rs-0014 */ #[feature(min_specialization)]
/* FP:lib.rs-0015 */ #[feature(never_type)]
/* FP:lib.rs-0016 */ #[feature(rustdoc_internals)]
/* FP:lib.rs-0017 */ #[feature(sized_hierarchy)]
/* FP:lib.rs-0018 */ // tidy-alphabetical-end
/* FP:lib.rs-0019 */ 
/* FP:lib.rs-0020 */ // Allows macros to refer to this crate as `::rustc_serialize`.
/* FP:lib.rs-0021 */ #[cfg(test)]
/* FP:lib.rs-0022 */ 
/* FP:lib.rs-0023 */ pub use self::serialize::{Decodable, Decoder, Encodable, Encoder};
/* FP:lib.rs-0024 */ 
/* FP:lib.rs-0026 */ 