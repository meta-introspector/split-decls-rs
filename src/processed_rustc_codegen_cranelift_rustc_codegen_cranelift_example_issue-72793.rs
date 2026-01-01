/* FP:issue-72793.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-72793_TRAIT_0001
/* FP:issue-72793.rs-0002 */ # [feature (type_alias_impl_trait)] pub trait T { type Item ; }
/* FP:issue-72793.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-72793_TYPE_0002
/* FP:issue-72793.rs-0004 */ pub type Alias < 'a > = impl T < Item = & 'a () > ;
/* FP:issue-72793.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-72793_STRUCT_0003
/* FP:issue-72793.rs-0006 */ struct S ;
/* FP:issue-72793.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-72793_IMPL_0004
/* FP:issue-72793.rs-0008 */ impl < 'a > T for & 'a S { type Item = & 'a () ; }
/* FP:issue-72793.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-72793_FN_0005
/* FP:issue-72793.rs-0010 */ # [define_opaque (Alias)] pub fn filter_positive < 'a > () -> Alias < 'a > { & S }
/* FP:issue-72793.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-72793_FN_0006
/* FP:issue-72793.rs-0012 */ fn with_positive (fun : impl Fn (Alias < '_ >)) { fun (filter_positive ()) ; }
/* FP:issue-72793.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-72793_FN_0007
/* FP:issue-72793.rs-0014 */ fn main () { with_positive (| _ | ()) ; }