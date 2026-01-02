mkuse!{use std :: sync :: Arc ;}
mkuse!{use annotate_snippets :: { Renderer , Snippet } ;}
mkuse!{use rustc_error_messages :: FluentArgs ;}
mkuse!{use rustc_span :: SourceFile ;}
mkuse!{use rustc_span :: source_map :: SourceMap ;}
mkuse!{use crate :: emitter :: FileWithAnnotatedLines ;}
mkuse!{use crate :: registry :: Registry ;}
mkuse!{use crate :: snippet :: Line ;}
mkuse!{use crate :: translation :: { Translator , to_fluent_args } ;}
mkuse!{use crate :: { CodeSuggestion , DiagInner , DiagMessage , Emitter , ErrCode , Level , MultiSpan , Style , Subdiag , } ;}
mkitem!{mkstruct!{# [doc = " Generates diagnostics using annotate-snippet"] pub struct AnnotateSnippetEmitter { source_map : Option < Arc < SourceMap > > , translator : Translator , # [doc = " If true, hides the longer explanation text"] short_message : bool , # [doc = " If true, will normalize line numbers with `LL` to prevent noise in UI test diffs."] ui_testing : bool , macro_backtrace : bool , }}}
mkitem!{mkimpl!{impl Emitter for AnnotateSnippetEmitter { # [doc = " The entry point for the diagnostics generation"] fn emit_diagnostic (& mut self , mut diag : DiagInner , _registry : & Registry) { let fluent_args = to_fluent_args (diag . args . iter ()) ; let mut suggestions = diag . suggestions . unwrap_tag () ; self . primary_span_formatted (& mut diag . span , & mut suggestions , & fluent_args) ; self . fix_multispans_in_extern_macros_and_render_macro_backtrace (& mut diag . span , & mut diag . children , & diag . level , self . macro_backtrace ,) ; self . emit_messages_default (& diag . level , & diag . messages , & fluent_args , & diag . code , & diag . span , & diag . children , & suggestions ,) ; } fn source_map (& self) -> Option < & SourceMap > { self . source_map . as_deref () } fn should_show_explain (& self) -> bool { ! self . short_message } fn translator (& self) -> & Translator { & self . translator } }}}

macro_rules! source_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function source_string in module {}", module_path!());
    };
}

mkfn!{
    source_string_introspect!();
    # [doc = " Provides the source string for the given `line` of `file`"] fn source_string (file : Arc < SourceFile > , line : & Line) -> String { file . get_line (line . line_index - 1) . map (| a | a . to_string ()) . unwrap_or_default () }
}

macro_rules! annotation_level_for_level_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function annotation_level_for_level in module {}", module_path!());
    };
}

mkfn!{
    annotation_level_for_level_introspect!();
    # [doc = " Maps [`crate::Level`] to [`annotate_snippets::Level`]"] fn annotation_level_for_level (level : Level) -> annotate_snippets :: Level { match level { Level :: Bug | Level :: Fatal | Level :: Error | Level :: DelayedBug => { annotate_snippets :: Level :: Error } Level :: ForceWarning | Level :: Warning => annotate_snippets :: Level :: Warning , Level :: Note | Level :: OnceNote => annotate_snippets :: Level :: Note , Level :: Help | Level :: OnceHelp => annotate_snippets :: Level :: Help , Level :: FailureNote => annotate_snippets :: Level :: Error , Level :: Allow => panic ! ("Should not call with Allow") , Level :: Expect => panic ! ("Should not call with Expect") , } }
}
mkitem!{mkimpl!{impl AnnotateSnippetEmitter { pub fn new (source_map : Option < Arc < SourceMap > > , translator : Translator , short_message : bool , macro_backtrace : bool ,) -> Self { Self { source_map , translator , short_message , ui_testing : false , macro_backtrace } } # [doc = " Allows to modify `Self` to enable or disable the `ui_testing` flag."] # [doc = ""] # [doc = " If this is set to true, line numbers will be normalized as `LL` in the output."] pub fn ui_testing (mut self , ui_testing : bool) -> Self { self . ui_testing = ui_testing ; self } fn emit_messages_default (& mut self , level : & Level , messages : & [(DiagMessage , Style)] , args : & FluentArgs < '_ > , code : & Option < ErrCode > , msp : & MultiSpan , _children : & [Subdiag] , _suggestions : & [CodeSuggestion] ,) { let message = self . translator . translate_messages (messages , args) ; if let Some (source_map) = & self . source_map { let primary_lo = if let Some (primary_span) = msp . primary_span () . as_ref () { if primary_span . is_dummy () { return ; } else { source_map . lookup_char_pos (primary_span . lo ()) } } else { return ; } ; let mut annotated_files = FileWithAnnotatedLines :: collect_annotations (self , args , msp) ; if let Ok (pos) = annotated_files . binary_search_by (| x | x . file . name . cmp (& primary_lo . file . name)) { annotated_files . swap (0 , pos) ; } type Owned = (String , String , usize , Vec < crate :: snippet :: Annotation >) ; let annotated_files : Vec < Owned > = annotated_files . into_iter () . flat_map (| annotated_file | { let file = annotated_file . file ; annotated_file . lines . into_iter () . map (| line | { source_map . ensure_source_file_source_present (& file) ; (format ! ("{}" , source_map . filename_for_diagnostics (& file . name)) , source_string (Arc :: clone (& file) , & line) , line . line_index , line . annotations ,) }) . collect :: < Vec < Owned > > () }) . collect () ; let code = code . map (| code | code . to_string ()) ; let snippets = annotated_files . iter () . map (| (file_name , source , line_index , annotations) | { Snippet :: source (source) . line_start (* line_index) . origin (file_name) . fold (false) . annotations (annotations . iter () . map (| annotation | { annotation_level_for_level (* level) . span (annotation . start_col . display .. annotation . end_col . display) . label (annotation . label . as_deref () . unwrap_or_default ()) })) }) ; let mut message = annotation_level_for_level (* level) . title (& message) . snippets (snippets) ; if let Some (code) = code . as_deref () { message = message . id (code) } let renderer = Renderer :: plain () . anonymized_line_numbers (self . ui_testing) ; eprintln ! ("{}" , renderer . render (message)) } } }}}