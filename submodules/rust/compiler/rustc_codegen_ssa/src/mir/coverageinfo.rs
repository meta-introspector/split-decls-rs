mkuse!{use rustc_middle :: mir :: SourceScope ;}
mkuse!{use rustc_middle :: mir :: coverage :: CoverageKind ;}
mkuse!{use super :: FunctionCx ;}
mkuse!{use crate :: traits :: * ;}
mkitem!{mkimpl!{impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > FunctionCx < 'a , 'tcx , Bx > { pub (crate) fn codegen_coverage (& self , bx : & mut Bx , kind : & CoverageKind , scope : SourceScope) { let instance = if let Some (inlined) = scope . inlined_instance (& self . mir . source_scopes) { self . monomorphize (inlined) } else { self . instance } ; bx . add_coverage (instance , kind) ; } }}}