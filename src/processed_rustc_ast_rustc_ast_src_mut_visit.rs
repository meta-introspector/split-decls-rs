/* FP:mut_visit.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_USE_0001
/* FP:mut_visit.rs-0002 */ use std :: ops :: DerefMut ;
/* FP:mut_visit.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_USE_0002
/* FP:mut_visit.rs-0004 */ use std :: panic ;
/* FP:mut_visit.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_USE_0003
/* FP:mut_visit.rs-0006 */ use crate :: rustc_data_structures :: flat_map_in_place :: FlatMapInPlace ;
/* FP:mut_visit.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_USE_0004
/* FP:mut_visit.rs-0008 */ use crate :: rustc_complete :: source_map :: Spanned ;
/* FP:mut_visit.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_USE_0005
/* FP:mut_visit.rs-0010 */ use crate :: rustc_complete :: { Ident , Span , Symbol } ;
/* FP:mut_visit.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_USE_0006
/* FP:mut_visit.rs-0012 */ use smallvec :: { SmallVec , smallvec } ;
/* FP:mut_visit.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_USE_0007
/* FP:mut_visit.rs-0014 */ use thin_vec :: ThinVec ;
/* FP:mut_visit.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_USE_0008
/* FP:mut_visit.rs-0016 */ use crate :: ast :: * ;
/* FP:mut_visit.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_USE_0009
/* FP:mut_visit.rs-0018 */ use crate :: tokenstream :: * ;
/* FP:mut_visit.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_USE_0010
/* FP:mut_visit.rs-0020 */ use crate :: visit :: { AssocCtxt , BoundKind , FnCtxt , LifetimeCtxt , VisitorResult , try_visit } ;
/* FP:mut_visit.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MOD_0011
/* FP:mut_visit.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_USE_0012
/* FP:mut_visit.rs-0024 */ use sealed :: MutVisitorResult ;
/* FP:mut_visit.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_TRAIT_0013
/* FP:mut_visit.rs-0026 */ pub (crate) trait MutVisitable < V : MutVisitor > { type Extra : Copy ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) ; }
/* FP:mut_visit.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_IMPL_0014
/* FP:mut_visit.rs-0028 */ impl < V : MutVisitor , T : ? Sized > MutVisitable < V > for Box < T > where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { (* * self) . visit_mut (visitor , extra) } }
/* FP:mut_visit.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_IMPL_0015
/* FP:mut_visit.rs-0030 */ impl < V : MutVisitor , T > MutVisitable < V > for Option < T > where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { if let Some (this) = self { this . visit_mut (visitor , extra) } } }
/* FP:mut_visit.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_IMPL_0016
/* FP:mut_visit.rs-0032 */ impl < V : MutVisitor , T > MutVisitable < V > for Spanned < T > where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { let Spanned { span , node } = self ; span . visit_mut (visitor , ()) ; node . visit_mut (visitor , extra) ; } }
/* FP:mut_visit.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_IMPL_0017
/* FP:mut_visit.rs-0034 */ impl < V : MutVisitor , T > MutVisitable < V > for [T] where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { for item in self { item . visit_mut (visitor , extra) ; } } }
/* FP:mut_visit.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_IMPL_0018
/* FP:mut_visit.rs-0036 */ impl < V : MutVisitor , T > MutVisitable < V > for Vec < T > where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { for item in self { item . visit_mut (visitor , extra) ; } } }
/* FP:mut_visit.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_IMPL_0019
/* FP:mut_visit.rs-0038 */ impl < V : MutVisitor , T > MutVisitable < V > for (T ,) where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { self . 0 . visit_mut (visitor , extra) ; } }
/* FP:mut_visit.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_IMPL_0020
/* FP:mut_visit.rs-0040 */ impl < V : MutVisitor , T1 , T2 > MutVisitable < V > for (T1 , T2) where T1 : MutVisitable < V , Extra = () > , T2 : MutVisitable < V , Extra = () > , { type Extra = () ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { self . 0 . visit_mut (visitor , extra) ; self . 1 . visit_mut (visitor , extra) ; } }
/* FP:mut_visit.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_IMPL_0021
/* FP:mut_visit.rs-0042 */ impl < V : MutVisitor , T1 , T2 , T3 > MutVisitable < V > for (T1 , T2 , T3) where T1 : MutVisitable < V , Extra = () > , T2 : MutVisitable < V , Extra = () > , T3 : MutVisitable < V , Extra = () > , { type Extra = () ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { self . 0 . visit_mut (visitor , extra) ; self . 1 . visit_mut (visitor , extra) ; self . 2 . visit_mut (visitor , extra) ; } }
/* FP:mut_visit.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_IMPL_0022
/* FP:mut_visit.rs-0044 */ impl < V : MutVisitor , T1 , T2 , T3 , T4 > MutVisitable < V > for (T1 , T2 , T3 , T4) where T1 : MutVisitable < V , Extra = () > , T2 : MutVisitable < V , Extra = () > , T3 : MutVisitable < V , Extra = () > , T4 : MutVisitable < V , Extra = () > , { type Extra = () ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { self . 0 . visit_mut (visitor , extra) ; self . 1 . visit_mut (visitor , extra) ; self . 2 . visit_mut (visitor , extra) ; self . 3 . visit_mut (visitor , extra) ; } }
/* FP:mut_visit.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_TRAIT_0023
/* FP:mut_visit.rs-0046 */ pub trait MutWalkable < V : MutVisitor > { fn walk_mut (& mut self , visitor : & mut V) ; }
/* FP:mut_visit.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0024
/* FP:mut_visit.rs-0048 */ macro_rules ! visit_visitable { (mut $ visitor : expr , $ ($ expr : expr) ,* $ (,) ?) => { { $ (MutVisitable :: visit_mut ($ expr , $ visitor , ()) ;) * } } ; }
/* FP:mut_visit.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0025
/* FP:mut_visit.rs-0050 */ macro_rules ! visit_visitable_with { (mut $ visitor : expr , $ expr : expr , $ extra : expr $ (,) ?) => { MutVisitable :: visit_mut ($ expr , $ visitor , $ extra) } ; }
/* FP:mut_visit.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0026
/* FP:mut_visit.rs-0052 */ macro_rules ! walk_walkable { ($ visitor : expr , $ expr : expr , mut) => { MutWalkable :: walk_mut ($ expr , $ visitor) } ; }
/* FP:mut_visit.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0027
/* FP:mut_visit.rs-0054 */ macro_rules ! impl_visitable { (|& mut $ self : ident : $ self_ty : ty , $ vis : ident : & mut $ vis_ty : ident , $ extra : ident : $ extra_ty : ty | $ block : block) => { # [allow (unused_parens , non_local_definitions)] impl <$ vis_ty : MutVisitor > MutVisitable <$ vis_ty > for $ self_ty { type Extra = $ extra_ty ; fn visit_mut (& mut $ self , $ vis : & mut $ vis_ty , $ extra : Self :: Extra) -> V :: Result { $ block } } } ; }
/* FP:mut_visit.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0028
/* FP:mut_visit.rs-0056 */ macro_rules ! impl_walkable { ($ (<$ K : ident : $ Kb : ident >) ? |& mut $ self : ident : $ self_ty : ty , $ vis : ident : & mut $ vis_ty : ident | $ block : block) => { # [allow (unused_parens , non_local_definitions)] impl <$ ($ K : $ Kb ,) ? $ vis_ty : MutVisitor > MutWalkable <$ vis_ty > for $ self_ty { fn walk_mut (& mut $ self , $ vis : & mut $ vis_ty) -> V :: Result { $ block } } } ; }
/* FP:mut_visit.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0029
/* FP:mut_visit.rs-0058 */ macro_rules ! impl_visitable_noop { (< mut > $ ($ ty : ty ,) *) => { $ (impl_visitable ! (|& mut self : $ ty , _vis : & mut V , _extra : () | { }) ;) * } ; }
/* FP:mut_visit.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0030
/* FP:mut_visit.rs-0060 */ macro_rules ! impl_visitable_list { (< mut > $ ($ ty : ty ,) *) => { $ (impl < V : MutVisitor , T > MutVisitable < V > for $ ty where for <'a > &'a mut $ ty : IntoIterator < Item = &'a mut T >, T : MutVisitable < V >, { type Extra = < T as MutVisitable < V >>:: Extra ; # [inline] fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { for i in self { i . visit_mut (visitor , extra) ; } } }) * } }
/* FP:mut_visit.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0031
/* FP:mut_visit.rs-0062 */ macro_rules ! impl_visitable_direct { (< mut > $ ($ ty : ty ,) *) => { $ (impl_visitable ! (|& mut self : $ ty , visitor : & mut V , _extra : () | { MutWalkable :: walk_mut (self , visitor) }) ;) * } }
/* FP:mut_visit.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0032
/* FP:mut_visit.rs-0064 */ macro_rules ! impl_visitable_calling_walkable { (< mut > $ (fn $ method : ident ($ ty : ty $ (, $ extra_name : ident : $ extra_ty : ty) ?) ;) *) => { $ (fn $ method (& mut self , node : & mut $ ty $ (, $ extra_name :$ extra_ty) ?) { impl_visitable ! (|& mut self : $ ty , visitor : & mut V , extra : ($ ($ extra_ty) ?) | { let ($ ($ extra_name) ?) = extra ; visitor .$ method (self $ (, $ extra_name) ?) ; }) ; walk_walkable ! (self , node , mut) }) * } }
/* FP:mut_visit.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0033
/* FP:mut_visit.rs-0066 */ macro_rules ! define_named_walk { ((mut) $ Visitor : ident $ (pub fn $ method : ident ($ ty : ty) ;) *) => { $ (pub fn $ method < V : $ Visitor > (visitor : & mut V , node : & mut $ ty) { walk_walkable ! (visitor , node , mut) }) * } ; }
/* FP:mut_visit.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0034
/* FP:mut_visit.rs-0068 */ super :: common_visitor_and_walkers ! ((mut) MutVisitor) ;
/* FP:mut_visit.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0035
/* FP:mut_visit.rs-0070 */ macro_rules ! generate_flat_map_visitor_fns { ($ ($ name : ident , $ Ty : ty , $ flat_map_fn : ident $ (, $ param : ident : $ ParamTy : ty) *;) +) => { $ (# [allow (unused_parens)] impl < V : MutVisitor > MutVisitable < V > for ThinVec <$ Ty > { type Extra = ($ ($ ParamTy) ,*) ; # [inline] fn visit_mut (& mut self , visitor : & mut V , ($ ($ param) ,*) : Self :: Extra ,) -> V :: Result { $ name (visitor , self $ (, $ param) *) } } fn $ name < V : MutVisitor > (vis : & mut V , values : & mut ThinVec <$ Ty >, $ ($ param : $ ParamTy ,) *) { values . flat_map_in_place (| value | vis .$ flat_map_fn (value $ (,$ param) *)) ; }) + } }
/* FP:mut_visit.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0036
/* FP:mut_visit.rs-0072 */ generate_flat_map_visitor_fns ! { visit_items , Box < Item >, flat_map_item ; visit_foreign_items , Box < ForeignItem >, flat_map_foreign_item ; visit_generic_params , GenericParam , flat_map_generic_param ; visit_stmts , Stmt , flat_map_stmt ; visit_exprs , Box < Expr >, filter_map_expr ; visit_expr_fields , ExprField , flat_map_expr_field ; visit_pat_fields , PatField , flat_map_pat_field ; visit_variants , Variant , flat_map_variant ; visit_assoc_items , Box < AssocItem >, flat_map_assoc_item , ctxt : AssocCtxt ; visit_where_predicates , WherePredicate , flat_map_where_predicate ; visit_params , Param , flat_map_param ; visit_field_defs , FieldDef , flat_map_field_def ; visit_arms , Arm , flat_map_arm ; }
/* FP:mut_visit.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_FN_0037
/* FP:mut_visit.rs-0074 */ pub fn walk_flat_map_pat_field < T : MutVisitor > (vis : & mut T , mut fp : PatField ,) -> SmallVec < [PatField ; 1] > { vis . visit_pat_field (& mut fp) ; smallvec ! [fp] }
/* FP:mut_visit.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0038
/* FP:mut_visit.rs-0076 */ macro_rules ! generate_walk_flat_map_fns { ($ ($ fn_name : ident ($ Ty : ty $ (,$ extra_name : ident : $ ExtraTy : ty) *) => $ visit_fn_name : ident ;) +) => { $ (pub fn $ fn_name < V : MutVisitor > (vis : & mut V , mut value : $ Ty $ (,$ extra_name : $ ExtraTy) *) -> SmallVec < [$ Ty ; 1] > { vis .$ visit_fn_name (& mut value $ (,$ extra_name) *) ; smallvec ! [value] }) + } ; }
/* FP:mut_visit.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_MACRO_0039
/* FP:mut_visit.rs-0078 */ generate_walk_flat_map_fns ! { walk_flat_map_arm (Arm) => visit_arm ; walk_flat_map_variant (Variant) => visit_variant ; walk_flat_map_param (Param) => visit_param ; walk_flat_map_generic_param (GenericParam) => visit_generic_param ; walk_flat_map_where_predicate (WherePredicate) => visit_where_predicate ; walk_flat_map_field_def (FieldDef) => visit_field_def ; walk_flat_map_expr_field (ExprField) => visit_expr_field ; walk_flat_map_item (Box < Item >) => visit_item ; walk_flat_map_foreign_item (Box < ForeignItem >) => visit_foreign_item ; walk_flat_map_assoc_item (Box < AssocItem >, ctxt : AssocCtxt) => visit_assoc_item ; }
/* FP:mut_visit.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_FN_0040
/* FP:mut_visit.rs-0080 */ pub fn walk_filter_map_expr < T : MutVisitor > (vis : & mut T , mut e : Box < Expr >) -> Option < Box < Expr > > { vis . visit_expr (& mut e) ; Some (e) }
/* FP:mut_visit.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_FN_0041
/* FP:mut_visit.rs-0082 */ pub fn walk_flat_map_stmt < T : MutVisitor > (vis : & mut T , Stmt { kind , span , mut id } : Stmt ,) -> SmallVec < [Stmt ; 1] > { vis . visit_id (& mut id) ; let mut stmts : SmallVec < [Stmt ; 1] > = walk_flat_map_stmt_kind (vis , kind) . into_iter () . map (| kind | Stmt { id , kind , span }) . collect () ; match & mut stmts [..] { [] => { } [stmt] => vis . visit_span (& mut stmt . span) , _ => panic ! ("cloning statement `NodeId`s is prohibited by default, \
/* FP:mut_visit.rs-0083 */              the visitor should implement custom statement visiting") , } stmts }
/* FP:mut_visit.rs-0084 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_mut_visit_FN_0042
/* FP:mut_visit.rs-0085 */ fn walk_flat_map_stmt_kind < T : MutVisitor > (vis : & mut T , kind : StmtKind) -> SmallVec < [StmtKind ; 1] > { match kind { StmtKind :: Let (mut local) => smallvec ! [StmtKind :: Let ({ vis . visit_local (& mut local) ; local })] , StmtKind :: Item (item) => vis . flat_map_item (item) . into_iter () . map (StmtKind :: Item) . collect () , StmtKind :: Expr (expr) => vis . filter_map_expr (expr) . into_iter () . map (StmtKind :: Expr) . collect () , StmtKind :: Semi (expr) => vis . filter_map_expr (expr) . into_iter () . map (StmtKind :: Semi) . collect () , StmtKind :: Empty => smallvec ! [StmtKind :: Empty] , StmtKind :: MacCall (mut mac) => { let MacCallStmt { mac : mac_ , style : _ , attrs , tokens : _ } = mac . deref_mut () ; for attr in attrs { vis . visit_attribute (attr) ; } vis . visit_mac_call (mac_) ; smallvec ! [StmtKind :: MacCall (mac)] } } }