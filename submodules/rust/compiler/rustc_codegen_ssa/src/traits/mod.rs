mkmod!{abi, { 
                getname!(abi);
                getsrc!(abi);
                getpath!(abi);
                get_deps!(abi);
                get_crates!(abi);
                mkinclude!(abi);
                 
            }}
mkmod!{asm, { 
                getname!(asm);
                getsrc!(asm);
                getpath!(asm);
                get_deps!(asm);
                get_crates!(asm);
                mkinclude!(asm);
                 
            }}
mkmod!{backend, { 
                getname!(backend);
                getsrc!(backend);
                getpath!(backend);
                get_deps!(backend);
                get_crates!(backend);
                mkinclude!(backend);
                 
            }}
mkmod!{builder, { 
                getname!(builder);
                getsrc!(builder);
                getpath!(builder);
                get_deps!(builder);
                get_crates!(builder);
                mkinclude!(builder);
                 
            }}
mkmod!{consts, { 
                getname!(consts);
                getsrc!(consts);
                getpath!(consts);
                get_deps!(consts);
                get_crates!(consts);
                mkinclude!(consts);
                 
            }}
mkmod!{coverageinfo, { 
                getname!(coverageinfo);
                getsrc!(coverageinfo);
                getpath!(coverageinfo);
                get_deps!(coverageinfo);
                get_crates!(coverageinfo);
                mkinclude!(coverageinfo);
                 
            }}
mkmod!{debuginfo, { 
                getname!(debuginfo);
                getsrc!(debuginfo);
                getpath!(debuginfo);
                get_deps!(debuginfo);
                get_crates!(debuginfo);
                mkinclude!(debuginfo);
                 
            }}
mkmod!{declare, { 
                getname!(declare);
                getsrc!(declare);
                getpath!(declare);
                get_deps!(declare);
                get_crates!(declare);
                mkinclude!(declare);
                 
            }}
mkmod!{intrinsic, { 
                getname!(intrinsic);
                getsrc!(intrinsic);
                getpath!(intrinsic);
                get_deps!(intrinsic);
                get_crates!(intrinsic);
                mkinclude!(intrinsic);
                 
            }}
mkmod!{misc, { 
                getname!(misc);
                getsrc!(misc);
                getpath!(misc);
                get_deps!(misc);
                get_crates!(misc);
                mkinclude!(misc);
                 
            }}
mkmod!{statics, { 
                getname!(statics);
                getsrc!(statics);
                getpath!(statics);
                get_deps!(statics);
                get_crates!(statics);
                mkinclude!(statics);
                 
            }}
mkmod!{type_, { 
                getname!(type_);
                getsrc!(type_);
                getpath!(type_);
                get_deps!(type_);
                get_crates!(type_);
                mkinclude!(type_);
                 
            }}
mkmod!{write, { 
                getname!(write);
                getsrc!(write);
                getpath!(write);
                get_deps!(write);
                get_crates!(write);
                mkinclude!(write);
                 
            }}
mkuse!{use std :: fmt ;}
mkuse!{use rustc_middle :: ty :: Ty ;}
mkuse!{use rustc_middle :: ty :: layout :: { FnAbiOf , LayoutOf , TyAndLayout } ;}
mkuse!{use rustc_target :: callconv :: FnAbi ;}
mkuse!{pub use self :: abi :: AbiBuilderMethods ;}
mkuse!{pub use self :: asm :: { AsmBuilderMethods , AsmCodegenMethods , GlobalAsmOperandRef , InlineAsmOperandRef , } ;}
mkuse!{pub use self :: backend :: { BackendTypes , CodegenBackend , ExtraBackendMethods } ;}
mkuse!{pub use self :: builder :: { BuilderMethods , OverflowOp } ;}
mkuse!{pub use self :: consts :: ConstCodegenMethods ;}
mkuse!{pub use self :: coverageinfo :: CoverageInfoBuilderMethods ;}
mkuse!{pub use self :: debuginfo :: { DebugInfoBuilderMethods , DebugInfoCodegenMethods } ;}
mkuse!{pub use self :: declare :: PreDefineCodegenMethods ;}
mkuse!{pub use self :: intrinsic :: IntrinsicCallBuilderMethods ;}
mkuse!{pub use self :: misc :: MiscCodegenMethods ;}
mkuse!{pub use self :: statics :: { StaticBuilderMethods , StaticCodegenMethods } ;}
mkuse!{pub use self :: type_ :: { ArgAbiBuilderMethods , BaseTypeCodegenMethods , DerivedTypeCodegenMethods , LayoutTypeCodegenMethods , TypeCodegenMethods , TypeMembershipCodegenMethods , } ;}
mkuse!{pub use self :: write :: { ModuleBufferMethods , ThinBufferMethods , WriteBackendMethods } ;}
mkitem!{pub trait CodegenObject = Copy + fmt :: Debug ;}
mkitem!{pub trait CodegenMethods < 'tcx > = LayoutOf < 'tcx , LayoutOfResult = TyAndLayout < 'tcx > > + FnAbiOf < 'tcx , FnAbiOfResult = & 'tcx FnAbi < 'tcx , Ty < 'tcx > > > + TypeCodegenMethods < 'tcx > + ConstCodegenMethods + StaticCodegenMethods + DebugInfoCodegenMethods < 'tcx > + AsmCodegenMethods < 'tcx > + PreDefineCodegenMethods < 'tcx > ;}