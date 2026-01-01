/* FP:lib.rs-0001 */ // The "main crate" of the Rust compiler. This crate contains common
/* FP:lib.rs-0002 */ // type definitions that are used by the other crates in the rustc
/* FP:lib.rs-0003 */ // "family". The following are some prominent examples.
/* FP:lib.rs-0004 */ //
/* FP:lib.rs-0005 */ // - **HIR.** The "high-level (H) intermediate representation (IR)" is
/* FP:lib.rs-0006 */ //   defined in the [`hir`] module.
/* FP:lib.rs-0007 */ // - **THIR.** The "typed high-level (H) intermediate representation (IR)"
/* FP:lib.rs-0008 */ //   is defined in the [`thir`] module.
/* FP:lib.rs-0009 */ // - **MIR.** The "mid-level (M) intermediate representation (IR)" is
/* FP:lib.rs-0010 */ //   defined in the [`mir`] module. This module contains only the
/* FP:lib.rs-0011 */ //   *definition* of the MIR; the passes that transform and operate
/* FP:lib.rs-0012 */ //   on MIR are found in `rustc_const_eval` crate.
/* FP:lib.rs-0013 */ // - **Types.** The internal representation of types used in rustc is
/* FP:lib.rs-0014 */ //   defined in the [`ty`] module. This includes the
/* FP:lib.rs-0015 */ //   [**type context**][ty::TyCtxt] (or `tcx`), which is the central
/* FP:lib.rs-0016 */ //   context during most of compilation, containing the interners and
/* FP:lib.rs-0017 */ //   other things.
/* FP:lib.rs-0018 */ //
/* FP:lib.rs-0019 */ // For more information about how rustc works, see the [rustc dev guide].
/* FP:lib.rs-0020 */ //
/* FP:lib.rs-0021 */ // [rustc dev guide]: https://rustc-dev-guide.rust-lang.org/
/* FP:lib.rs-0022 */ //
/* FP:lib.rs-0023 */ // # Note
/* FP:lib.rs-0024 */ //
/* FP:lib.rs-0025 */ // This API is completely unstable and subject to change.
/* FP:lib.rs-0026 */ 
/* FP:lib.rs-0027 */ // tidy-alphabetical-start
/* FP:lib.rs-0028 */ #[allow(internal_features)]
/* FP:lib.rs-0029 */ #[allow(rustc::diagnostic_outside_of_impl)]
/* FP:lib.rs-0030 */ #[allow(rustc::direct_use_of_rustc_type_ir)]
/* FP:lib.rs-0031 */ #[allow(rustc::untranslatable_diagnostic)]
/* FP:lib.rs-0032 */ #[cfg_attr(bootstrap, feature(round_char_boundary))]
/* FP:lib.rs-0033 */ #[doc(html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")]
/* FP:lib.rs-0034 */ #[doc(rust_logo)]
/* FP:lib.rs-0035 */ #[feature(allocator_api)]
/* FP:lib.rs-0036 */ #[feature(array_windows)]
/* FP:lib.rs-0037 */ #[feature(assert_matches)]
/* FP:lib.rs-0038 */ #[feature(associated_type_defaults)]
/* FP:lib.rs-0039 */ #[feature(box_as_ptr)]
/* FP:lib.rs-0040 */ #[feature(box_patterns)]
/* FP:lib.rs-0041 */ #[feature(closure_track_caller)]
/* FP:lib.rs-0042 */ #[feature(core_intrinsics)]
/* FP:lib.rs-0043 */ #[feature(debug_closure_helpers)]
/* FP:lib.rs-0044 */ #[feature(decl_macro)]
/* FP:lib.rs-0045 */ #[feature(discriminant_kind)]
/* FP:lib.rs-0046 */ #[feature(extern_types)]
/* FP:lib.rs-0047 */ #[feature(file_buffered)]
/* FP:lib.rs-0048 */ #[feature(gen_blocks)]
/* FP:lib.rs-0049 */ #[feature(if_let_guard)]
/* FP:lib.rs-0050 */ #[feature(intra_doc_pointers)]
/* FP:lib.rs-0051 */ #[feature(min_specialization)]
/* FP:lib.rs-0052 */ #[feature(negative_impls)]
/* FP:lib.rs-0053 */ #[feature(never_type)]
/* FP:lib.rs-0054 */ #[feature(ptr_alignment_type)]
/* FP:lib.rs-0055 */ #[feature(rustc_attrs)]
/* FP:lib.rs-0056 */ #[feature(rustdoc_internals)]
/* FP:lib.rs-0057 */ #[feature(sized_hierarchy)]
/* FP:lib.rs-0058 */ #[feature(try_blocks)]
/* FP:lib.rs-0059 */ #[feature(try_trait_v2)]
/* FP:lib.rs-0060 */ #[feature(try_trait_v2_residual)]
/* FP:lib.rs-0061 */ #[feature(try_trait_v2_yeet)]
/* FP:lib.rs-0062 */ #[feature(type_alias_impl_trait)]
/* FP:lib.rs-0063 */ #[feature(unwrap_infallible)]
/* FP:lib.rs-0064 */ #[feature(yeet_expr)]
/* FP:lib.rs-0065 */ #[recursion_limit = "256"]
/* FP:lib.rs-0066 */ // tidy-alphabetical-end
/* FP:lib.rs-0067 */ 
/* FP:lib.rs-0068 */ #[cfg(test)]
/* FP:lib.rs-0070 */ 
/* FP:lib.rs-0071 */ #[macro_use]
/* FP:lib.rs-0073 */ 
/* FP:lib.rs-0074 */ #[macro_use]
/* FP:lib.rs-0089 */ 
/* FP:lib.rs-0090 */ #[macro_use]
/* FP:lib.rs-0092 */ #[macro_use]
/* FP:lib.rs-0094 */ 
/* FP:lib.rs-0095 */ // Allows macros to refer to this crate as `::rustc_middle`
/* FP:lib.rs-0096 */ 
/* FP:lib.rs-0097 */ rustc_fluent_macro::fluent_messages! { "../messages.ftl" }