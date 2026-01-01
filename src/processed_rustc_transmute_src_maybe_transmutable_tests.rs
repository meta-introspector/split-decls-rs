/* FP:tests.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_USE_0001
/* FP:tests.rs-0002 */ use itertools :: Itertools ;
/* FP:tests.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_USE_0002
/* FP:tests.rs-0004 */ use super :: query_context :: test :: { Def , UltraMinimal } ;
/* FP:tests.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_USE_0003
/* FP:tests.rs-0006 */ use crate :: { Answer , Assume , Condition , Reason , layout } ;
/* FP:tests.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_TYPE_0004
/* FP:tests.rs-0008 */ type Tree = layout :: Tree < Def , ! , ! > ;
/* FP:tests.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_TYPE_0005
/* FP:tests.rs-0010 */ type Dfa = layout :: Dfa < ! , ! > ;
/* FP:tests.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_TRAIT_0006
/* FP:tests.rs-0012 */ trait Representation { fn is_transmutable (src : Self , dst : Self , assume : Assume) -> Answer < ! , ! > ; }
/* FP:tests.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_IMPL_0007
/* FP:tests.rs-0014 */ impl Representation for Tree { fn is_transmutable (src : Self , dst : Self , assume : Assume) -> Answer < ! , ! > { crate :: maybe_transmutable :: MaybeTransmutableQuery :: new (src , dst , assume , UltraMinimal :: default () ,) . answer () } }
/* FP:tests.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_IMPL_0008
/* FP:tests.rs-0016 */ impl Representation for Dfa { fn is_transmutable (src : Self , dst : Self , assume : Assume) -> Answer < ! , ! > { crate :: maybe_transmutable :: MaybeTransmutableQuery :: new (src , dst , assume , UltraMinimal :: default () ,) . answer () } }
/* FP:tests.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_FN_0009
/* FP:tests.rs-0018 */ fn is_transmutable < R : Representation + Clone > (src : & R , dst : & R , assume : Assume ,) -> crate :: Answer < ! , ! > { let src = src . clone () ; let dst = dst . clone () ; R :: is_transmutable (src , dst , assume) }
/* FP:tests.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_MOD_0010
/* FP:tests.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_MOD_0011
/* FP:tests.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_MOD_0012
/* FP:tests.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_MOD_0013
/* FP:tests.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_MOD_0014
/* FP:tests.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_MOD_0015
/* FP:tests.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_MOD_0016
/* FP:tests.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_MOD_0017
/* FP:tests.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_MOD_0018
/* FP:tests.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_transmute_src_maybe_transmutable_tests_MOD_0019