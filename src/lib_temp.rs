
#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(rustc_private)]
#![feature(never_type)]
#![feature(rustc_attrs)]
#![feature(lang_items)]
#![feature(optimize_attribute)]
#![feature(allocator_api)]

extern crate tracing;
extern crate synstructure;
extern crate proc_macro;

include!("wrap_types.rs");

pub mod rustc_macros {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Decodable;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Decodable_NoContext;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Encodable;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Encodable_NoContext;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct HashStable_Generic;
}

pub mod rustc_hashes {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Hash64;
}

pub mod reg {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Reg;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct RegKind;
}

pub mod tracing {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct debug;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct trace;
}

pub mod canon_abi {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct ArmCall;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct CanonAbi;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct InterruptKind;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct X86Call;
}

pub mod bitflags {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct bitflags;
}

pub mod layout {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct FIRST_VARIANT;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct FieldIdx;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Layout;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct LayoutCalculator;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct LayoutCalculatorError;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct TyAbiInterface;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct TyAndLayout;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct VariantIdx;
}

pub mod extern_abi {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct CVariadicStatus;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct ExternAbi;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct all_names;
}

pub mod ty {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct FIRST_VARIANT;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct FieldIdx;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Layout;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct TyAbiInterface;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct TyAndLayout;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct VariantIdx;
}

pub mod callconv {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Heterogeneous;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct HomogeneousAggregate;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Reg;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct RegKind;
}


include!("processed_rustc_abi_src_layout_simple.rs");
