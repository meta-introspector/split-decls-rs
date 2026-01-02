mkuse!{use rustc_codegen_ssa :: traits :: CoverageInfoBuilderMethods ;}
mkuse!{use rustc_middle :: mir :: coverage :: CoverageKind ;}
mkuse!{use rustc_middle :: ty :: Instance ;}
mkuse!{use crate :: builder :: Builder ;}
mkitem!{mkimpl!{impl < 'a , 'gcc , 'tcx > CoverageInfoBuilderMethods < 'tcx > for Builder < 'a , 'gcc , 'tcx > { fn add_coverage (& mut self , _instance : Instance < 'tcx > , _kind : & CoverageKind) { } }}}