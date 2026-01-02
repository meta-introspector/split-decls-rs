mkuse!{use std :: str :: FromStr ;}
mkuse!{use rustc_abi :: { Align , ExternAbi } ;}
mkuse!{use rustc_ast :: expand :: autodiff_attrs :: { AutoDiffAttrs , DiffActivity , DiffMode } ;}
mkuse!{use rustc_ast :: { LitKind , MetaItem , MetaItemInner , attr } ;}
mkuse!{use rustc_hir :: attrs :: { AttributeKind , InlineAttr , InstructionSetAttr , UsedBy } ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LOCAL_CRATE , LocalDefId } ;}
mkuse!{use rustc_hir :: { self as hir , Attribute , LangItem , find_attr , lang_items } ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: { CodegenFnAttrFlags , CodegenFnAttrs , PatchableFunctionEntry , } ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: ty :: { self as ty , TyCtxt } ;}
mkuse!{use rustc_session :: lint ;}
mkuse!{use rustc_session :: parse :: feature_err ;}
mkuse!{use rustc_span :: { Ident , Span , sym } ;}
mkuse!{use rustc_target :: spec :: SanitizerSet ;}
mkuse!{use crate :: errors ;}
mkuse!{use crate :: errors :: NoMangleNameless ;}
mkuse!{use crate :: target_features :: { check_target_feature_trait_unsafe , check_tied_features , from_target_feature_attr , } ;}

macro_rules! try_fn_sig_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_fn_sig in module {}", module_path!());
    };
}

mkfn!{
    try_fn_sig_introspect!();
    # [doc = " In some cases, attributes are only valid on functions, but it's the `check_attr`"] # [doc = " pass that checks that they aren't used anywhere else, rather than this module."] # [doc = " In these cases, we bail from performing further checks that are only meaningful for"] # [doc = " functions (such as calling `fn_sig`, which ICEs if given a non-function). We also"] # [doc = " report a delayed bug, just in case `check_attr` isn't doing its job."] fn try_fn_sig < 'tcx > (tcx : TyCtxt < 'tcx > , did : LocalDefId , attr_span : Span ,) -> Option < ty :: EarlyBinder < 'tcx , ty :: PolyFnSig < 'tcx > > > { use DefKind :: * ; let def_kind = tcx . def_kind (did) ; if let Fn | AssocFn | Variant | Ctor (..) = def_kind { Some (tcx . fn_sig (did)) } else { tcx . dcx () . span_delayed_bug (attr_span , "this attribute can only be applied to functions") ; None } }
}

macro_rules! parse_instruction_set_attr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_instruction_set_attr in module {}", module_path!());
    };
}

mkfn!{
    parse_instruction_set_attr_introspect!();
    fn parse_instruction_set_attr (tcx : TyCtxt < '_ > , attr : & Attribute) -> Option < InstructionSetAttr > { let list = attr . meta_item_list () ? ; match & list [..] { [MetaItemInner :: MetaItem (set)] => { let segments = set . path . segments . iter () . map (| x | x . ident . name) . collect :: < Vec < _ > > () ; match segments . as_slice () { [sym :: arm , sym :: a32 | sym :: t32] if ! tcx . sess . target . has_thumb_interworking => { tcx . dcx () . emit_err (errors :: UnsupportedInstructionSet { span : attr . span () }) ; None } [sym :: arm , sym :: a32] => Some (InstructionSetAttr :: ArmA32) , [sym :: arm , sym :: t32] => Some (InstructionSetAttr :: ArmT32) , _ => { tcx . dcx () . emit_err (errors :: InvalidInstructionSet { span : attr . span () }) ; None } } } [] => { tcx . dcx () . emit_err (errors :: BareInstructionSet { span : attr . span () }) ; None } _ => { tcx . dcx () . emit_err (errors :: MultipleInstructionSet { span : attr . span () }) ; None } } }
}

macro_rules! parse_patchable_function_entry_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_patchable_function_entry in module {}", module_path!());
    };
}

mkfn!{
    parse_patchable_function_entry_introspect!();
    fn parse_patchable_function_entry (tcx : TyCtxt < '_ > , attr : & Attribute ,) -> Option < PatchableFunctionEntry > { attr . meta_item_list () . and_then (| l | { let mut prefix = None ; let mut entry = None ; for item in l { let Some (meta_item) = item . meta_item () else { tcx . dcx () . emit_err (errors :: ExpectedNameValuePair { span : item . span () }) ; continue ; } ; let Some (name_value_lit) = meta_item . name_value_literal () else { tcx . dcx () . emit_err (errors :: ExpectedNameValuePair { span : item . span () }) ; continue ; } ; let attrib_to_write = match meta_item . name () { Some (sym :: prefix_nops) => & mut prefix , Some (sym :: entry_nops) => & mut entry , _ => { tcx . dcx () . emit_err (errors :: UnexpectedParameterName { span : item . span () , prefix_nops : sym :: prefix_nops , entry_nops : sym :: entry_nops , }) ; continue ; } } ; let rustc_ast :: LitKind :: Int (val , _) = name_value_lit . kind else { tcx . dcx () . emit_err (errors :: InvalidLiteralValue { span : name_value_lit . span }) ; continue ; } ; let Ok (val) = val . get () . try_into () else { tcx . dcx () . emit_err (errors :: OutOfRangeInteger { span : name_value_lit . span }) ; continue ; } ; * attrib_to_write = Some (val) ; } if let (None , None) = (prefix , entry) { tcx . dcx () . span_err (attr . span () , "must specify at least one parameter") ; } Some (PatchableFunctionEntry :: from_prefix_and_entry (prefix . unwrap_or (0) , entry . unwrap_or (0))) }) }
}
mkitem!{mkstruct!{# [doc = " Spans that are collected when processing built-in attributes,"] # [doc = " that are useful for emitting diagnostics later."] # [derive (Default)] struct InterestingAttributeDiagnosticSpans { link_ordinal : Option < Span > , sanitize : Option < Span > , inline : Option < Span > , no_mangle : Option < Span > , }}}

macro_rules! process_builtin_attrs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function process_builtin_attrs in module {}", module_path!());
    };
}

mkfn!{
    process_builtin_attrs_introspect!();
    # [doc = " Process the builtin attrs ([`hir::Attribute`]) on the item."] # [doc = " Many of them directly translate to codegen attrs."] fn process_builtin_attrs (tcx : TyCtxt < '_ > , did : LocalDefId , attrs : & [Attribute] , codegen_fn_attrs : & mut CodegenFnAttrs ,) -> InterestingAttributeDiagnosticSpans { let mut interesting_spans = InterestingAttributeDiagnosticSpans :: default () ; let rust_target_features = tcx . rust_target_features (LOCAL_CRATE) ; for attr in attrs . iter () { if let hir :: Attribute :: Parsed (p) = attr { match p { AttributeKind :: Cold (_) => codegen_fn_attrs . flags |= CodegenFnAttrFlags :: COLD , AttributeKind :: ExportName { name , .. } => { codegen_fn_attrs . symbol_name = Some (* name) } AttributeKind :: Inline (inline , span) => { codegen_fn_attrs . inline = * inline ; interesting_spans . inline = Some (* span) ; } AttributeKind :: Naked (_) => codegen_fn_attrs . flags |= CodegenFnAttrFlags :: NAKED , AttributeKind :: Align { align , .. } => codegen_fn_attrs . alignment = Some (* align) , AttributeKind :: LinkName { name , .. } => { if tcx . is_foreign_item (did) { codegen_fn_attrs . symbol_name = Some (* name) ; } } AttributeKind :: LinkOrdinal { ordinal , span } => { codegen_fn_attrs . link_ordinal = Some (* ordinal) ; interesting_spans . link_ordinal = Some (* span) ; } AttributeKind :: LinkSection { name , .. } => { codegen_fn_attrs . link_section = Some (* name) } AttributeKind :: NoMangle (attr_span) => { interesting_spans . no_mangle = Some (* attr_span) ; if tcx . opt_item_name (did . to_def_id ()) . is_some () { codegen_fn_attrs . flags |= CodegenFnAttrFlags :: NO_MANGLE ; } else { tcx . dcx () . emit_err (NoMangleNameless { span : * attr_span , definition : format ! ("{} {}" , tcx . def_descr_article (did . to_def_id ()) , tcx . def_descr (did . to_def_id ())) , }) ; } } AttributeKind :: Optimize (optimize , _) => codegen_fn_attrs . optimize = * optimize , AttributeKind :: TargetFeature { features , attr_span , was_forced } => { let Some (sig) = tcx . hir_node_by_def_id (did) . fn_sig () else { tcx . dcx () . span_delayed_bug (* attr_span , "target_feature applied to non-fn") ; continue ; } ; let safe_target_features = matches ! (sig . header . safety , hir :: HeaderSafety :: SafeTargetFeatures) ; codegen_fn_attrs . safe_target_features = safe_target_features ; if safe_target_features && ! was_forced { if tcx . sess . target . is_like_wasm || tcx . sess . opts . actually_rustdoc { } else { check_target_feature_trait_unsafe (tcx , did , * attr_span) ; } } from_target_feature_attr (tcx , did , features , * was_forced , rust_target_features , & mut codegen_fn_attrs . target_features ,) ; } AttributeKind :: TrackCaller (attr_span) => { let is_closure = tcx . is_closure_like (did . to_def_id ()) ; if ! is_closure && let Some (fn_sig) = try_fn_sig (tcx , did , * attr_span) && fn_sig . skip_binder () . abi () != ExternAbi :: Rust { tcx . dcx () . emit_err (errors :: RequiresRustAbi { span : * attr_span }) ; } if is_closure && ! tcx . features () . closure_track_caller () && ! attr_span . allows_unstable (sym :: closure_track_caller) { feature_err (& tcx . sess , sym :: closure_track_caller , * attr_span , "`#[track_caller]` on closures is currently unstable" ,) . emit () ; } codegen_fn_attrs . flags |= CodegenFnAttrFlags :: TRACK_CALLER } AttributeKind :: Used { used_by , .. } => match used_by { UsedBy :: Compiler => codegen_fn_attrs . flags |= CodegenFnAttrFlags :: USED_COMPILER , UsedBy :: Linker => codegen_fn_attrs . flags |= CodegenFnAttrFlags :: USED_LINKER , } , AttributeKind :: FfiConst (_) => { codegen_fn_attrs . flags |= CodegenFnAttrFlags :: FFI_CONST } AttributeKind :: FfiPure (_) => codegen_fn_attrs . flags |= CodegenFnAttrFlags :: FFI_PURE , AttributeKind :: StdInternalSymbol (_) => { codegen_fn_attrs . flags |= CodegenFnAttrFlags :: RUSTC_STD_INTERNAL_SYMBOL } AttributeKind :: Linkage (linkage , _) => { let linkage = Some (* linkage) ; if tcx . is_foreign_item (did) { codegen_fn_attrs . import_linkage = linkage ; if tcx . is_mutable_static (did . into ()) { let mut diag = tcx . dcx () . struct_span_err (attr . span () , "extern mutable statics are not allowed with `#[linkage]`" ,) ; diag . note ("marking the extern static mutable would allow changing which \
                                symbol the static references rather than make the target of the \
                                symbol mutable" ,) ; diag . emit () ; } } else { codegen_fn_attrs . linkage = linkage ; } } AttributeKind :: Sanitize { span , .. } => { interesting_spans . sanitize = Some (* span) ; } _ => { } } } let Some (Ident { name , .. }) = attr . ident () else { continue ; } ; match name { sym :: rustc_allocator => codegen_fn_attrs . flags |= CodegenFnAttrFlags :: ALLOCATOR , sym :: rustc_nounwind => codegen_fn_attrs . flags |= CodegenFnAttrFlags :: NEVER_UNWIND , sym :: rustc_reallocator => codegen_fn_attrs . flags |= CodegenFnAttrFlags :: REALLOCATOR , sym :: rustc_deallocator => codegen_fn_attrs . flags |= CodegenFnAttrFlags :: DEALLOCATOR , sym :: rustc_allocator_zeroed => { codegen_fn_attrs . flags |= CodegenFnAttrFlags :: ALLOCATOR_ZEROED } sym :: thread_local => codegen_fn_attrs . flags |= CodegenFnAttrFlags :: THREAD_LOCAL , sym :: instruction_set => { codegen_fn_attrs . instruction_set = parse_instruction_set_attr (tcx , attr) } sym :: patchable_function_entry => { codegen_fn_attrs . patchable_function_entry = parse_patchable_function_entry (tcx , attr) ; } _ => { } } } interesting_spans }
}

macro_rules! apply_overrides_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function apply_overrides in module {}", module_path!());
    };
}

mkfn!{
    apply_overrides_introspect!();
    # [doc = " Applies overrides for codegen fn attrs. These often have a specific reason why they're necessary."] # [doc = " Please comment why when adding a new one!"] fn apply_overrides (tcx : TyCtxt < '_ > , did : LocalDefId , codegen_fn_attrs : & mut CodegenFnAttrs) { codegen_fn_attrs . alignment = Ord :: max (codegen_fn_attrs . alignment , tcx . sess . opts . unstable_opts . min_function_alignment) ; codegen_fn_attrs . no_sanitize |= tcx . disabled_sanitizers_for (did) ; codegen_fn_attrs . alignment = Ord :: max (codegen_fn_attrs . alignment , tcx . inherited_align (did)) ; if codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: NAKED) { codegen_fn_attrs . inline = InlineAttr :: Never ; } if tcx . is_closure_like (did . to_def_id ()) && codegen_fn_attrs . inline != InlineAttr :: Always { let owner_id = tcx . parent (did . to_def_id ()) ; if tcx . def_kind (owner_id) . has_codegen_attrs () { codegen_fn_attrs . target_features . extend (tcx . codegen_fn_attrs (owner_id) . target_features . iter () . copied ()) ; } } let crate_attrs = tcx . hir_attrs (rustc_hir :: CRATE_HIR_ID) ; let no_builtins = attr :: contains_name (crate_attrs , sym :: no_builtins) ; if no_builtins { codegen_fn_attrs . flags |= CodegenFnAttrFlags :: NO_BUILTINS ; } if tcx . should_inherit_track_caller (did) { codegen_fn_attrs . flags |= CodegenFnAttrFlags :: TRACK_CALLER ; } if tcx . is_foreign_item (did) { codegen_fn_attrs . flags |= CodegenFnAttrFlags :: FOREIGN_ITEM ; if codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: RUSTC_STD_INTERNAL_SYMBOL) { } else if codegen_fn_attrs . symbol_name . is_some () { } else { codegen_fn_attrs . flags |= CodegenFnAttrFlags :: NO_MANGLE ; } } }
}

macro_rules! check_result_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_result in module {}", module_path!());
    };
}

mkfn!{
    check_result_introspect!();
    fn check_result (tcx : TyCtxt < '_ > , did : LocalDefId , interesting_spans : InterestingAttributeDiagnosticSpans , codegen_fn_attrs : & CodegenFnAttrs ,) { if ! codegen_fn_attrs . target_features . is_empty () && matches ! (codegen_fn_attrs . inline , InlineAttr :: Always) && ! tcx . features () . target_feature_inline_always () && let Some (span) = interesting_spans . inline { feature_err (tcx . sess , sym :: target_feature_inline_always , span , "cannot use `#[inline(always)]` with `#[target_feature]`" ,) . emit () ; } if ! codegen_fn_attrs . no_sanitize . is_empty () && codegen_fn_attrs . inline . always () && let (Some (no_sanitize_span) , Some (inline_span)) = (interesting_spans . sanitize , interesting_spans . inline) { let hir_id = tcx . local_def_id_to_hir_id (did) ; tcx . node_span_lint (lint :: builtin :: INLINE_NO_SANITIZE , hir_id , no_sanitize_span , | lint | { lint . primary_message ("setting `sanitize` off will have no effect after inlining") ; lint . span_note (inline_span , "inlining requested here") ; }) } if let Some (_) = codegen_fn_attrs . symbol_name && let Some (_) = codegen_fn_attrs . link_ordinal { let msg = "cannot use `#[link_name]` with `#[link_ordinal]`" ; if let Some (span) = interesting_spans . link_ordinal { tcx . dcx () . span_err (span , msg) ; } else { tcx . dcx () . err (msg) ; } } if let Some (features) = check_tied_features (tcx . sess , & codegen_fn_attrs . target_features . iter () . map (| features | (features . name . as_str () , true)) . collect () ,) { let span = find_attr ! (tcx . get_all_attrs (did) , AttributeKind :: TargetFeature { attr_span : span , .. } => * span) . unwrap_or_else (| | tcx . def_span (did)) ; tcx . dcx () . create_err (errors :: TargetFeatureDisableOrEnable { features , span : Some (span) , missing_features : Some (errors :: MissingFeatures) , }) . emit () ; } }
}

macro_rules! handle_lang_items_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function handle_lang_items in module {}", module_path!());
    };
}

mkfn!{
    handle_lang_items_introspect!();
    fn handle_lang_items (tcx : TyCtxt < '_ > , did : LocalDefId , interesting_spans : & InterestingAttributeDiagnosticSpans , attrs : & [Attribute] , codegen_fn_attrs : & mut CodegenFnAttrs ,) { let lang_item = lang_items :: extract (attrs) . and_then (| (name , _) | LangItem :: from_name (name)) ; if let Some (lang_item) = lang_item && let Some (link_name) = lang_item . link_name () { codegen_fn_attrs . flags |= CodegenFnAttrFlags :: RUSTC_STD_INTERNAL_SYMBOL ; codegen_fn_attrs . symbol_name = Some (link_name) ; } if codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: RUSTC_STD_INTERNAL_SYMBOL) && codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: NO_MANGLE) { let mut err = tcx . dcx () . struct_span_err (interesting_spans . no_mangle . unwrap_or_default () , "`#[no_mangle]` cannot be used on internal language items" ,) . with_note ("Rustc requires this item to have a specific mangled name.") . with_span_label (tcx . def_span (did) , "should be the internal language item") ; if let Some (lang_item) = lang_item && let Some (link_name) = lang_item . link_name () { err = err . with_note ("If you are trying to prevent mangling to ease debugging, many") . with_note (format ! ("debuggers support a command such as `rbreak {link_name}` to")) . with_note (format ! ("match `.*{link_name}.*` instead of `break {link_name}` on a specific name")) } err . emit () ; } }
}

macro_rules! codegen_fn_attrs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_fn_attrs in module {}", module_path!());
    };
}

mkfn!{
    codegen_fn_attrs_introspect!();
    # [doc = " Generate the [`CodegenFnAttrs`] for an item (identified by the [`LocalDefId`])."] # [doc = ""] # [doc = " This happens in 4 stages:"] # [doc = " - apply built-in attributes that directly translate to codegen attributes."] # [doc = " - handle lang items. These have special codegen attrs applied to them."] # [doc = " - apply overrides, like minimum requirements for alignment and other settings that don't rely directly the built-in attrs on the item."] # [doc = "   overrides come after applying built-in attributes since they may only apply when certain attributes were already set in the stage before."] # [doc = " - check that the result is valid. There's various ways in which this may not be the case, such as certain combinations of attrs."] fn codegen_fn_attrs (tcx : TyCtxt < '_ > , did : LocalDefId) -> CodegenFnAttrs { if cfg ! (debug_assertions) { let def_kind = tcx . def_kind (did) ; assert ! (def_kind . has_codegen_attrs () , "unexpected `def_kind` in `codegen_fn_attrs`: {def_kind:?}" ,) ; } let mut codegen_fn_attrs = CodegenFnAttrs :: new () ; let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (did)) ; let interesting_spans = process_builtin_attrs (tcx , did , attrs , & mut codegen_fn_attrs) ; handle_lang_items (tcx , did , & interesting_spans , attrs , & mut codegen_fn_attrs) ; apply_overrides (tcx , did , & mut codegen_fn_attrs) ; check_result (tcx , did , interesting_spans , & codegen_fn_attrs) ; codegen_fn_attrs }
}

macro_rules! disabled_sanitizers_for_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function disabled_sanitizers_for in module {}", module_path!());
    };
}

mkfn!{
    disabled_sanitizers_for_introspect!();
    fn disabled_sanitizers_for (tcx : TyCtxt < '_ > , did : LocalDefId) -> SanitizerSet { let mut disabled = match tcx . opt_local_parent (did) { Some (parent) => tcx . disabled_sanitizers_for (parent) , None => SanitizerSet :: empty () , } ; if let Some ((on_set , off_set)) = find_attr ! (tcx . get_all_attrs (did) , AttributeKind :: Sanitize { on_set , off_set , .. } => (on_set , off_set)) { disabled &= ! * on_set ; disabled |= * off_set ; } disabled }
}

macro_rules! should_inherit_track_caller_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function should_inherit_track_caller in module {}", module_path!());
    };
}

mkfn!{
    should_inherit_track_caller_introspect!();
    # [doc = " Checks if the provided DefId is a method in a trait impl for a trait which has track_caller"] # [doc = " applied to the method prototype."] fn should_inherit_track_caller (tcx : TyCtxt < '_ > , def_id : DefId) -> bool { tcx . trait_item_of (def_id) . is_some_and (| id | { tcx . codegen_fn_attrs (id) . flags . intersects (CodegenFnAttrFlags :: TRACK_CALLER) }) }
}

macro_rules! inherited_align_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inherited_align in module {}", module_path!());
    };
}

mkfn!{
    inherited_align_introspect!();
    # [doc = " If the provided DefId is a method in a trait impl, return the value of the `#[align]`"] # [doc = " attribute on the method prototype (if any)."] fn inherited_align < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId) -> Option < Align > { tcx . codegen_fn_attrs (tcx . trait_item_of (def_id) ?) . alignment }
}

macro_rules! autodiff_attrs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function autodiff_attrs in module {}", module_path!());
    };
}

mkfn!{
    autodiff_attrs_introspect!();
    # [doc = " We now check the #\\[rustc_autodiff\\] attributes which we generated from the #[autodiff(...)]"] # [doc = " macros. There are two forms. The pure one without args to mark primal functions (the functions"] # [doc = " being differentiated). The other form is #[rustc_autodiff(Mode, ActivityList)] on top of the"] # [doc = " placeholder functions. We wrote the rustc_autodiff attributes ourself, so this should never"] # [doc = " panic, unless we introduced a bug when parsing the autodiff macro."] pub fn autodiff_attrs (tcx : TyCtxt < '_ > , id : DefId) -> Option < AutoDiffAttrs > { let attrs = tcx . get_attrs (id , sym :: rustc_autodiff) ; let attrs = attrs . filter (| attr | attr . has_name (sym :: rustc_autodiff)) . collect :: < Vec < _ > > () ; let attr = match & attrs [..] { [] => return None , [attr] => attr , _ => { span_bug ! (attrs [1] . span () , "cg_ssa: rustc_autodiff should only exist once per source") ; } } ; let list = attr . meta_item_list () . unwrap_or_default () ; if list . is_empty () { return Some (AutoDiffAttrs :: source ()) ; } let [mode , width_meta , input_activities @ .. , ret_activity] = & list [..] else { span_bug ! (attr . span () , "rustc_autodiff attribute must contain mode, width and activities") ; } ; let mode = if let MetaItemInner :: MetaItem (MetaItem { path : p1 , .. }) = mode { p1 . segments . first () . unwrap () . ident } else { span_bug ! (attr . span () , "rustc_autodiff attribute must contain mode") ; } ; let mode = match mode . as_str () { "Forward" => DiffMode :: Forward , "Reverse" => DiffMode :: Reverse , _ => { span_bug ! (mode . span , "rustc_autodiff attribute contains invalid mode") ; } } ; let width : u32 = match width_meta { MetaItemInner :: MetaItem (MetaItem { path : p1 , .. }) => { let w = p1 . segments . first () . unwrap () . ident ; match w . as_str () . parse () { Ok (val) => val , Err (_) => { span_bug ! (w . span , "rustc_autodiff width should fit u32") ; } } } MetaItemInner :: Lit (lit) => { if let LitKind :: Int (val , _) = lit . kind { match val . get () . try_into () { Ok (val) => val , Err (_) => { span_bug ! (lit . span , "rustc_autodiff width should fit u32") ; } } } else { span_bug ! (lit . span , "rustc_autodiff width should be an integer") ; } } } ; let ret_symbol = if let MetaItemInner :: MetaItem (MetaItem { path : p1 , .. }) = ret_activity { p1 . segments . first () . unwrap () . ident } else { span_bug ! (attr . span () , "rustc_autodiff attribute must contain the return activity") ; } ; let Ok (ret_activity) = DiffActivity :: from_str (ret_symbol . as_str ()) else { span_bug ! (ret_symbol . span , "invalid return activity") ; } ; let mut arg_activities : Vec < DiffActivity > = vec ! [] ; for arg in input_activities { let arg_symbol = if let MetaItemInner :: MetaItem (MetaItem { path : p2 , .. }) = arg { match p2 . segments . first () { Some (x) => x . ident , None => { span_bug ! (arg . span () , "rustc_autodiff attribute must contain the input activity") ; } } } else { span_bug ! (arg . span () , "rustc_autodiff attribute must contain the input activity") ; } ; match DiffActivity :: from_str (arg_symbol . as_str ()) { Ok (arg_activity) => arg_activities . push (arg_activity) , Err (_) => { span_bug ! (arg_symbol . span , "invalid input activity") ; } } } Some (AutoDiffAttrs { mode , width , ret_activity , input_activity : arg_activities }) }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { codegen_fn_attrs , should_inherit_track_caller , inherited_align , disabled_sanitizers_for , .. * providers } ; }
}