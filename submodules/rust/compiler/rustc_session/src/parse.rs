mkuse!{use std :: str ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use rustc_ast :: attr :: AttrIdGenerator ;}
mkuse!{use rustc_ast :: node_id :: NodeId ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_data_structures :: sync :: { AppendOnlyVec , Lock } ;}
mkuse!{use rustc_errors :: emitter :: { FatalOnlyEmitter , HumanEmitter , stderr_destination } ;}
mkuse!{use rustc_errors :: translation :: Translator ;}
mkuse!{use rustc_errors :: { BufferedEarlyLint , ColorConfig , DecorateDiagCompat , Diag , DiagCtxt , DiagCtxtHandle , DiagMessage , EmissionGuarantee , MultiSpan , StashKey , } ;}
mkuse!{use rustc_feature :: { GateIssue , UnstableFeatures , find_feature_issue } ;}
mkuse!{use rustc_span :: edition :: Edition ;}
mkuse!{use rustc_span :: hygiene :: ExpnId ;}
mkuse!{use rustc_span :: source_map :: { FilePathMapping , SourceMap } ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use crate :: Session ;}
mkuse!{use crate :: config :: { Cfg , CheckCfg } ;}
mkuse!{use crate :: errors :: { CliFeatureDiagnosticHelp , FeatureDiagnosticForIssue , FeatureDiagnosticHelp , FeatureDiagnosticSuggestion , FeatureGateError , SuggestUpgradeCompiler , } ;}
mkuse!{use crate :: lint :: builtin :: UNSTABLE_SYNTAX_PRE_EXPANSION ;}
mkuse!{use crate :: lint :: { Lint , LintId } ;}
mkitem!{mkstruct!{# [doc = " Collected spans during parsing for places where a certain feature was"] # [doc = " used and should be feature gated accordingly in `check_crate`."] # [derive (Default)] pub struct GatedSpans { pub spans : Lock < FxHashMap < Symbol , Vec < Span > > > , }}}
mkitem!{mkimpl!{impl GatedSpans { # [doc = " Feature gate the given `span` under the given `feature`"] # [doc = " which is same `Symbol` used in `unstable.rs`."] pub fn gate (& self , feature : Symbol , span : Span) { self . spans . borrow_mut () . entry (feature) . or_default () . push (span) ; } # [doc = " Ungate the last span under the given `feature`."] # [doc = " Panics if the given `span` wasn't the last one."] # [doc = ""] # [doc = " Using this is discouraged unless you have a really good reason to."] pub fn ungate_last (& self , feature : Symbol , span : Span) { let removed_span = self . spans . borrow_mut () . entry (feature) . or_default () . pop () . unwrap () ; debug_assert_eq ! (span , removed_span) ; } # [doc = " Prepend the given set of `spans` onto the set in `self`."] pub fn merge (& self , mut spans : FxHashMap < Symbol , Vec < Span > >) { let mut inner = self . spans . borrow_mut () ; # [allow (rustc :: potential_query_instability)] for (gate , mut gate_spans) in inner . drain () { spans . entry (gate) . or_default () . append (& mut gate_spans) ; } * inner = spans ; } }}}
mkitem!{mkstruct!{# [derive (Default)] pub struct SymbolGallery { # [doc = " All symbols occurred and their first occurrence span."] pub symbols : Lock < FxIndexMap < Symbol , Span > > , }}}
mkitem!{mkimpl!{impl SymbolGallery { # [doc = " Insert a symbol and its span into symbol gallery."] # [doc = " If the symbol has occurred before, ignore the new occurrence."] pub fn insert (& self , symbol : Symbol , span : Span) { self . symbols . lock () . entry (symbol) . or_insert (span) ; } }}}

macro_rules! feature_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function feature_err in module {}", module_path!());
    };
}

mkfn!{
    feature_err_introspect!();
    # [doc = " Construct a diagnostic for a language feature error due to the given `span`."] # [doc = " The `feature`'s `Symbol` is the one you used in `unstable.rs` and `rustc_span::symbol`."] # [track_caller] pub fn feature_err (sess : & Session , feature : Symbol , span : impl Into < MultiSpan > , explain : impl Into < DiagMessage > ,) -> Diag < '_ > { feature_err_issue (sess , feature , span , GateIssue :: Language , explain) }
}

macro_rules! feature_err_issue_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function feature_err_issue in module {}", module_path!());
    };
}

mkfn!{
    feature_err_issue_introspect!();
    # [doc = " Construct a diagnostic for a feature gate error."] # [doc = ""] # [doc = " This variant allows you to control whether it is a library or language feature."] # [doc = " Almost always, you want to use this for a language feature. If so, prefer `feature_err`."] # [track_caller] pub fn feature_err_issue (sess : & Session , feature : Symbol , span : impl Into < MultiSpan > , issue : GateIssue , explain : impl Into < DiagMessage > ,) -> Diag < '_ > { let span = span . into () ; if let Some (span) = span . primary_span () && let Some (err) = sess . dcx () . steal_non_err (span , StashKey :: EarlySyntaxWarning) { err . cancel () } let mut err = sess . dcx () . create_err (FeatureGateError { span , explain : explain . into () }) ; add_feature_diagnostics_for_issue (& mut err , sess , feature , issue , false , None) ; err }
}

macro_rules! feature_warn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function feature_warn in module {}", module_path!());
    };
}

mkfn!{
    feature_warn_introspect!();
    # [doc = " Construct a future incompatibility diagnostic for a feature gate."] # [doc = ""] # [doc = " This diagnostic is only a warning and *does not cause compilation to fail*."] # [track_caller] pub fn feature_warn (sess : & Session , feature : Symbol , span : Span , explain : & 'static str) { feature_warn_issue (sess , feature , span , GateIssue :: Language , explain) ; }
}

macro_rules! feature_warn_issue_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function feature_warn_issue in module {}", module_path!());
    };
}

mkfn!{
    feature_warn_issue_introspect!();
    # [doc = " Construct a future incompatibility diagnostic for a feature gate."] # [doc = ""] # [doc = " This diagnostic is only a warning and *does not cause compilation to fail*."] # [doc = ""] # [doc = " This variant allows you to control whether it is a library or language feature."] # [doc = " Almost always, you want to use this for a language feature. If so, prefer `feature_warn`."] # [allow (rustc :: diagnostic_outside_of_impl)] # [allow (rustc :: untranslatable_diagnostic)] # [track_caller] pub fn feature_warn_issue (sess : & Session , feature : Symbol , span : Span , issue : GateIssue , explain : & 'static str ,) { let mut err = sess . dcx () . struct_span_warn (span , explain) ; add_feature_diagnostics_for_issue (& mut err , sess , feature , issue , false , None) ; let lint = UNSTABLE_SYNTAX_PRE_EXPANSION ; let future_incompatible = lint . future_incompatible . as_ref () . unwrap () ; err . is_lint (lint . name_lower () , false) ; err . warn (lint . desc) ; err . note (format ! ("for more information, see {}" , future_incompatible . reference)) ; err . stash (span , StashKey :: EarlySyntaxWarning) ; }
}

macro_rules! add_feature_diagnostics_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_feature_diagnostics in module {}", module_path!());
    };
}

mkfn!{
    add_feature_diagnostics_introspect!();
    # [doc = " Adds the diagnostics for a feature to an existing error."] # [doc = " Must be a language feature!"] pub fn add_feature_diagnostics < G : EmissionGuarantee > (err : & mut Diag < '_ , G > , sess : & Session , feature : Symbol ,) { add_feature_diagnostics_for_issue (err , sess , feature , GateIssue :: Language , false , None) ; }
}

macro_rules! add_feature_diagnostics_for_issue_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_feature_diagnostics_for_issue in module {}", module_path!());
    };
}

mkfn!{
    add_feature_diagnostics_for_issue_introspect!();
    # [doc = " Adds the diagnostics for a feature to an existing error."] # [doc = ""] # [doc = " This variant allows you to control whether it is a library or language feature."] # [doc = " Almost always, you want to use this for a language feature. If so, prefer"] # [doc = " `add_feature_diagnostics`."] # [allow (rustc :: diagnostic_outside_of_impl)] pub fn add_feature_diagnostics_for_issue < G : EmissionGuarantee > (err : & mut Diag < '_ , G > , sess : & Session , feature : Symbol , issue : GateIssue , feature_from_cli : bool , inject_span : Option < Span > ,) { if let Some (n) = find_feature_issue (feature , issue) { err . subdiagnostic (FeatureDiagnosticForIssue { n }) ; } if sess . psess . unstable_features . is_nightly_build () { if feature_from_cli { err . subdiagnostic (CliFeatureDiagnosticHelp { feature }) ; } else if let Some (span) = inject_span { err . subdiagnostic (FeatureDiagnosticSuggestion { feature , span }) ; } else { err . subdiagnostic (FeatureDiagnosticHelp { feature }) ; } if feature == sym :: rustc_attrs { } else if sess . opts . unstable_opts . ui_testing { err . subdiagnostic (SuggestUpgradeCompiler :: ui_testing ()) ; } else if let Some (suggestion) = SuggestUpgradeCompiler :: new () { err . subdiagnostic (suggestion) ; } } }
}

macro_rules! feature_err_unstable_feature_bound_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function feature_err_unstable_feature_bound in module {}", module_path!());
    };
}

mkfn!{
    feature_err_unstable_feature_bound_introspect!();
    # [doc = " This is only used by unstable_feature_bound as it does not have issue number information for now."] # [doc = " This is basically the same as `feature_err_issue`"] # [doc = " but without the feature issue note. If we can do a lookup for issue number from feature name,"] # [doc = " then we should directly use `feature_err_issue` for ambiguity error of"] # [doc = " `#[unstable_feature_bound]`."] # [track_caller] pub fn feature_err_unstable_feature_bound (sess : & Session , feature : Symbol , span : impl Into < MultiSpan > , explain : impl Into < DiagMessage > ,) -> Diag < '_ > { let span = span . into () ; if let Some (span) = span . primary_span () { if let Some (err) = sess . dcx () . steal_non_err (span , StashKey :: EarlySyntaxWarning) { err . cancel () } } let mut err = sess . dcx () . create_err (FeatureGateError { span , explain : explain . into () }) ; if sess . psess . unstable_features . is_nightly_build () { err . subdiagnostic (FeatureDiagnosticHelp { feature }) ; if feature == sym :: rustc_attrs { } else if sess . opts . unstable_opts . ui_testing { err . subdiagnostic (SuggestUpgradeCompiler :: ui_testing ()) ; } else if let Some (suggestion) = SuggestUpgradeCompiler :: new () { err . subdiagnostic (suggestion) ; } } err }
}
mkitem!{mkstruct!{# [doc = " Info about a parsing session."] pub struct ParseSess { dcx : DiagCtxt , pub unstable_features : UnstableFeatures , pub config : Cfg , pub check_config : CheckCfg , pub edition : Edition , # [doc = " Places where raw identifiers were used. This is used to avoid complaining about idents"] # [doc = " clashing with keywords in new editions."] pub raw_identifier_spans : AppendOnlyVec < Span > , # [doc = " Places where identifiers that contain invalid Unicode codepoints but that look like they"] # [doc = " should be. Useful to avoid bad tokenization when encountering emoji. We group them to"] # [doc = " provide a single error per unique incorrect identifier."] pub bad_unicode_identifiers : Lock < FxIndexMap < Symbol , Vec < Span > > > , source_map : Arc < SourceMap > , pub buffered_lints : Lock < Vec < BufferedEarlyLint > > , # [doc = " Contains the spans of block expressions that could have been incomplete based on the"] # [doc = " operation token that followed it, but that the parser cannot identify without further"] # [doc = " analysis."] pub ambiguous_block_expr_parse : Lock < FxIndexMap < Span , Span > > , pub gated_spans : GatedSpans , pub symbol_gallery : SymbolGallery , # [doc = " Environment variables accessed during the build and their values when they exist."] pub env_depinfo : Lock < FxIndexSet < (Symbol , Option < Symbol >) > > , # [doc = " File paths accessed during the build."] pub file_depinfo : Lock < FxIndexSet < Symbol > > , # [doc = " Whether cfg(version) should treat the current release as incomplete"] pub assume_incomplete_release : bool , # [doc = " Spans passed to `proc_macro::quote_span`. Each span has a numerical"] # [doc = " identifier represented by its position in the vector."] proc_macro_quoted_spans : AppendOnlyVec < Span > , # [doc = " Used to generate new `AttrId`s. Every `AttrId` is unique."] pub attr_id_generator : AttrIdGenerator , }}}
mkitem!{mkimpl!{impl ParseSess { # [doc = " Used for testing."] pub fn new (locale_resources : Vec < & 'static str >) -> Self { let translator = Translator :: with_fallback_bundle (locale_resources , false) ; let sm = Arc :: new (SourceMap :: new (FilePathMapping :: empty ())) ; let emitter = Box :: new (HumanEmitter :: new (stderr_destination (ColorConfig :: Auto) , translator) . sm (Some (Arc :: clone (& sm))) ,) ; let dcx = DiagCtxt :: new (emitter) ; ParseSess :: with_dcx (dcx , sm) } pub fn with_dcx (dcx : DiagCtxt , source_map : Arc < SourceMap >) -> Self { Self { dcx , unstable_features : UnstableFeatures :: from_environment (None) , config : Cfg :: default () , check_config : CheckCfg :: default () , edition : ExpnId :: root () . expn_data () . edition , raw_identifier_spans : Default :: default () , bad_unicode_identifiers : Lock :: new (Default :: default ()) , source_map , buffered_lints : Lock :: new (vec ! []) , ambiguous_block_expr_parse : Lock :: new (Default :: default ()) , gated_spans : GatedSpans :: default () , symbol_gallery : SymbolGallery :: default () , env_depinfo : Default :: default () , file_depinfo : Default :: default () , assume_incomplete_release : false , proc_macro_quoted_spans : Default :: default () , attr_id_generator : AttrIdGenerator :: new () , } } pub fn with_fatal_emitter (locale_resources : Vec < & 'static str > , fatal_note : String) -> Self { let translator = Translator :: with_fallback_bundle (locale_resources , false) ; let sm = Arc :: new (SourceMap :: new (FilePathMapping :: empty ())) ; let fatal_emitter = Box :: new (HumanEmitter :: new (stderr_destination (ColorConfig :: Auto) , translator)) ; let dcx = DiagCtxt :: new (Box :: new (FatalOnlyEmitter { fatal_emitter , fatal_note : Some (fatal_note) , })) . disable_warnings () ; ParseSess :: with_dcx (dcx , sm) } # [inline] pub fn source_map (& self) -> & SourceMap { & self . source_map } pub fn clone_source_map (& self) -> Arc < SourceMap > { Arc :: clone (& self . source_map) } pub fn buffer_lint (& self , lint : & 'static Lint , span : impl Into < MultiSpan > , node_id : NodeId , diagnostic : impl Into < DecorateDiagCompat > ,) { self . opt_span_buffer_lint (lint , Some (span . into ()) , node_id , diagnostic . into ()) } pub (crate) fn opt_span_buffer_lint (& self , lint : & 'static Lint , span : Option < MultiSpan > , node_id : NodeId , diagnostic : DecorateDiagCompat ,) { self . buffered_lints . with_lock (| buffered_lints | { buffered_lints . push (BufferedEarlyLint { span , node_id , lint_id : LintId :: of (lint) , diagnostic , }) ; }) ; } pub fn save_proc_macro_span (& self , span : Span) -> usize { self . proc_macro_quoted_spans . push (span) } pub fn proc_macro_quoted_spans (& self) -> impl Iterator < Item = (usize , Span) > { self . proc_macro_quoted_spans . iter_enumerated () } pub fn dcx (& self) -> DiagCtxtHandle < '_ > { self . dcx . handle () } }}}