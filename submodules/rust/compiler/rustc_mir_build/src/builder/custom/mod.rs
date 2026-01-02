mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: { HirId , attrs } ;}
mkuse!{use rustc_index :: { IndexSlice , IndexVec } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: mir :: * ;}
mkuse!{use rustc_middle :: thir :: * ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_span :: Span ;}
mkmod!{parse, { 
                getname!(parse);
                getsrc!(parse);
                getpath!(parse);
                get_deps!(parse);
                get_crates!(parse);
                mkinclude!(parse);
                 
            }}

macro_rules! build_custom_mir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_custom_mir in module {}", module_path!());
    };
}

mkfn!{
    build_custom_mir_introspect!();
    pub (super) fn build_custom_mir < 'tcx > (tcx : TyCtxt < 'tcx > , did : DefId , hir_id : HirId , thir : & Thir < 'tcx > , expr : ExprId , params : & IndexSlice < ParamId , Param < 'tcx > > , return_ty : Ty < 'tcx > , return_ty_span : Span , span : Span , dialect : Option < attrs :: MirDialect > , phase : Option < attrs :: MirPhase > ,) -> Body < 'tcx > { let mut body = Body { basic_blocks : BasicBlocks :: new (IndexVec :: new ()) , source : MirSource :: item (did) , phase : MirPhase :: Built , source_scopes : IndexVec :: new () , coroutine : None , local_decls : IndexVec :: new () , user_type_annotations : IndexVec :: new () , arg_count : params . len () , spread_arg : None , var_debug_info : Vec :: new () , span , required_consts : None , mentioned_items : None , is_polymorphic : false , tainted_by_errors : None , injection_phase : None , pass_count : 0 , coverage_info_hi : None , function_coverage_info : None , } ; body . local_decls . push (LocalDecl :: new (return_ty , return_ty_span)) ; body . basic_blocks_mut () . push (BasicBlockData :: new (None , false)) ; body . source_scopes . push (SourceScopeData { span , parent_scope : None , inlined : None , inlined_parent_scope : None , local_data : ClearCrossCrate :: Set (SourceScopeLocalData { lint_root : hir_id }) , }) ; body . injection_phase = Some (parse_attribute (dialect , phase)) ; let mut pctxt = ParseCtxt { tcx , typing_env : body . typing_env (tcx) , thir , source_scope : OUTERMOST_SOURCE_SCOPE , body : & mut body , local_map : FxHashMap :: default () , block_map : FxHashMap :: default () , } ; let res : PResult < _ > = try { pctxt . parse_args (params) ? ; pctxt . parse_body (expr) ? ; } ; if let Err (err) = res { tcx . dcx () . span_fatal (err . span , format ! ("Could not parse {}, found: {:?}" , err . expected , err . item_description) ,) } body }
}

macro_rules! parse_attribute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_attribute in module {}", module_path!());
    };
}

mkfn!{
    parse_attribute_introspect!();
    # [doc = " Turns the arguments passed to `#[custom_mir(..)]` into a proper"] # [doc = " [`MirPhase`]. Panics if this isn't possible for any reason."] fn parse_attribute (dialect : Option < attrs :: MirDialect > , phase : Option < attrs :: MirPhase >) -> MirPhase { let Some (dialect) = dialect else { assert ! (phase . is_none ()) ; return MirPhase :: Built ; } ; match dialect { attrs :: MirDialect :: Built => { assert ! (phase . is_none () , "Cannot specify a phase for `Built` MIR") ; MirPhase :: Built } attrs :: MirDialect :: Analysis => match phase { None | Some (attrs :: MirPhase :: Initial) => MirPhase :: Analysis (AnalysisPhase :: Initial) , Some (attrs :: MirPhase :: PostCleanup) => MirPhase :: Analysis (AnalysisPhase :: PostCleanup) , Some (attrs :: MirPhase :: Optimized) => { bug ! ("`optimized` dialect is not compatible with the `analysis` dialect") } } , attrs :: MirDialect :: Runtime => match phase { None | Some (attrs :: MirPhase :: Initial) => MirPhase :: Runtime (RuntimePhase :: Initial) , Some (attrs :: MirPhase :: PostCleanup) => MirPhase :: Runtime (RuntimePhase :: PostCleanup) , Some (attrs :: MirPhase :: Optimized) => MirPhase :: Runtime (RuntimePhase :: Optimized) , } , } }
}
mkitem!{mkstruct!{struct ParseCtxt < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , thir : & 'a Thir < 'tcx > , source_scope : SourceScope , body : & 'a mut Body < 'tcx > , local_map : FxHashMap < LocalVarId , Local > , block_map : FxHashMap < LocalVarId , BasicBlock > , }}}
mkitem!{mkstruct!{struct ParseError { span : Span , item_description : String , expected : String , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > ParseCtxt < 'a , 'tcx > { fn expr_error (& self , expr : ExprId , expected : & 'static str) -> ParseError { let expr = & self . thir [expr] ; ParseError { span : expr . span , item_description : format ! ("{:?}" , expr . kind) , expected : expected . to_string () , } } fn stmt_error (& self , stmt : StmtId , expected : & 'static str) -> ParseError { let stmt = & self . thir [stmt] ; let span = match stmt . kind { StmtKind :: Expr { expr , .. } => self . thir [expr] . span , StmtKind :: Let { span , .. } => span , } ; ParseError { span , item_description : format ! ("{:?}" , stmt . kind) , expected : expected . to_string () , } } }}}
mkitem!{type PResult < T > = Result < T , ParseError > ;}