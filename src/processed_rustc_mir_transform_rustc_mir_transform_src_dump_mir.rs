/* FP:dump_mir.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_dump_mir_USE_0001
/* FP:dump_mir.rs-0002 */ use std :: fs :: File ;
/* FP:dump_mir.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_dump_mir_USE_0002
/* FP:dump_mir.rs-0004 */ use std :: io ;
/* FP:dump_mir.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_dump_mir_USE_0003
/* FP:dump_mir.rs-0006 */ use crate :: rustc_complete :: mir :: { Body , write_mir_pretty } ;
/* FP:dump_mir.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_dump_mir_USE_0004
/* FP:dump_mir.rs-0008 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:dump_mir.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_dump_mir_USE_0005
/* FP:dump_mir.rs-0010 */ use crate :: rustc_complete :: config :: { OutFileName , OutputType } ;
/* FP:dump_mir.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_dump_mir_STRUCT_0006
/* FP:dump_mir.rs-0012 */ pub (super) struct Marker (pub & 'static str) ;
/* FP:dump_mir.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_dump_mir_IMPL_0007
/* FP:dump_mir.rs-0014 */ impl < 'tcx > crate :: MirPass < 'tcx > for Marker { fn name (& self) -> & 'static str { self . 0 } fn run_pass (& self , _tcx : TyCtxt < 'tcx > , _body : & mut Body < 'tcx >) { } fn is_required (& self) -> bool { false } }
/* FP:dump_mir.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_dump_mir_FN_0008
/* FP:dump_mir.rs-0016 */ pub fn emit_mir (tcx : TyCtxt < '_ >) -> io :: Result < () > { match tcx . output_filenames (()) . path (OutputType :: Mir) { OutFileName :: Stdout => { let mut f = io :: stdout () ; write_mir_pretty (tcx , None , & mut f) ? ; } OutFileName :: Real (path) => { let mut f = File :: create_buffered (& path) ? ; write_mir_pretty (tcx , None , & mut f) ? ; if tcx . sess . opts . json_artifact_notifications { tcx . dcx () . emit_artifact_notification (& path , "mir") ; } } } Ok (()) }